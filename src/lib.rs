mod element;
mod error;
mod grid;
mod ticker;
mod universe;

use crate::{
    element::Element,
    error::JsResult,
    grid::Grid,
    ticker::Ticker,
    universe::{LifeState, Universe},
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, HtmlInputElement};

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
    let play_pause_btn = Button::get_by_id("play/pause").unwrap();
    let step_restart_btn = Button::get_by_id("step/restart").unwrap();
    let speed_slider = Input::get_by_id("speed").unwrap();

    let ticker = Rc::new(Ticker::new(
        {
            let universe = universe.clone();
            let play_pause_btn = play_pause_btn.clone();

            universe.borrow_mut().tick();
            move || {
                let mut universe = universe.borrow_mut();
                let success = (grid.update(&universe).js_ok())
                    .filter(|()| universe.is_alive())
                    .map(|()| universe.tick())
                    .is_some();
                if !success {
                    play_pause_btn.set_disabled(true);
                }
                success
            }
        },
        speed_slider.get_delay(),
    ));

    step_restart_btn.on("click", {
        let universe = universe.clone();
        let ticker = ticker.clone();
        let play_pause_btn = play_pause_btn.clone();
        let p_alive_slider = p_alive_slider.clone();

        move |_| {
            if ticker.is_running() || !universe.borrow().is_alive() {
                universe
                    .borrow_mut()
                    .randomize(p_alive_slider.get_p_alive());
                if !ticker.is_running() {
                    ticker.start();
                    play_pause_btn.set_disabled(false);
                }
            } else {
                ticker.step();
            }
        }
    });

    play_pause_btn.on("click", {
        let ticker = ticker.clone();
        let step_restart_btn = step_restart_btn.clone();

        move |this| {
            if ticker.is_running() {
                this.set_text_content("Play".into());
                step_restart_btn.set_text_content("Step".into());
                ticker.stop();
            } else {
                this.set_text_content("Pause".into());
                step_restart_btn.set_text_content("Restart".into());
                ticker.start();
            }
        }
    });

    speed_slider.on("input", {
        let ticker = ticker.clone();
        move |this| ticker.set_delay(this.get_delay())
    });

    p_alive_slider.on("input", move |this| {
        ticker.stop();
        universe.borrow_mut().randomize(this.get_p_alive());
        ticker.step();

        play_pause_btn.set_text_content("Play".into());
        step_restart_btn.set_text_content("Step".into());
        play_pause_btn.set_disabled(false);
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
        // html value is as speed; speed = -delay
        #[allow(clippy::cast_possible_truncation)] // we control the HTML value, it won't overflow
        let delay = -self.value_as_number() as i32;
        delay
    }
}
