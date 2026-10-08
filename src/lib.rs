mod element;
mod grid;
mod ticker;
mod universe;

use crate::{
    element::Element,
    grid::Grid,
    ticker::Ticker,
    universe::{LifeState, Universe},
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, HtmlInputElement, console};

const UNIVERSE_SIZE: [u32; 2] = [64, 64];
const CELL_SIZE: u32 = 10;
const LINE_THICKNESS: u32 = 1;

const fn cell_color(cell: Option<LifeState>) -> [u8; 4] {
    match cell {
        None => [64, 61, 82, 255],
        Some(LifeState::Alive) => [224, 222, 224, 255],
        Some(LifeState::Dying) => [235, 111, 146, 255],
        Some(LifeState::Dead) => [25, 23, 36, 255],
    }
}

/// # Panics
/// May panic if anything goes wrong with the DOM
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let p_alive_slider = Input::get_by_id("p_alive").unwrap();
    let universe = Rc::new(RefCell::new(Universe::new(
        UNIVERSE_SIZE,
        p_alive_slider.get_p_alive(),
    )));
    let mut grid = Grid::new(
        "canvas",
        &universe.borrow(),
        CELL_SIZE,
        LINE_THICKNESS,
        cell_color,
    )
    .unwrap();
    let play_btn = Button::get_by_id("play-pause").unwrap();
    let step_btn = Button::get_by_id("step").unwrap();
    let speed_slider = Input::get_by_id("speed").unwrap();

    let ticker = Rc::new(Ticker::new(
        {
            let universe = universe.clone();
            let play_btn = play_btn.clone();
            let step_btn = step_btn.clone();
            let speed_slider = speed_slider.clone();

            universe.borrow_mut().tick();
            move || {
                let to_continue = if let Err(e) = grid.update(&universe.borrow()) {
                    console::error_1(&e);
                    false
                } else if !universe.borrow().is_alive() {
                    false
                } else {
                    universe.borrow_mut().tick();
                    true
                };
                if !to_continue {
                    play_btn.set_disabled(true);
                    step_btn.set_disabled(true);
                    speed_slider.set_disabled(true);
                }
                to_continue
            }
        },
        speed_slider.get_delay(),
    ));

    step_btn.on("click", {
        let ticker = ticker.clone();
        move |_| ticker.step()
    });

    play_btn.on("click", {
        let ticker = ticker.clone();
        let step_btn = step_btn.clone();
        let speed_slider = speed_slider.clone();

        move |this| {
            let was_running = ticker.is_running();
            step_btn.set_disabled(!was_running);
            speed_slider.set_disabled(was_running);

            if was_running {
                this.set_text_content("Play".into());
                ticker.stop();
            } else {
                this.set_text_content("Pause".into());
                ticker.start();
            }
        }
    });

    speed_slider.on("input", {
        let ticker = ticker.clone();
        move |this| {
            #[allow(clippy::cast_possible_truncation)]
            ticker.set_delay(this.get_delay());
        }
    });

    p_alive_slider.on("input", move |this| {
        ticker.stop();
        universe.borrow_mut().randomize(this.get_p_alive());
        ticker.step();

        play_btn.set_text_content("Play".into());
        play_btn.set_disabled(false);
        step_btn.set_disabled(false);
        speed_slider.set_disabled(true);
    });
}

type Button = Element<HtmlButtonElement>;
type Input = Element<HtmlInputElement>;

impl Input {
    fn get_p_alive(&self) -> f64 {
        // html value is in percentage, convert it to decimal
        self.value_as_number() / 100.
    }

    fn get_delay(&self) -> i32 {
        // html value is in terms of speed; speed = -delay
        #[allow(clippy::cast_possible_truncation)] // we control the HTML value, it won't overflow
        let delay = -self.value_as_number() as i32;
        delay
    }
}
