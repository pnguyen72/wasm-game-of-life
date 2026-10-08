mod element;
mod grid;
mod ticker;
mod universe;

use crate::{
    element::{button::Button, input::Input},
    grid::Grid,
    ticker::Ticker,
    universe::Universe,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use wasm_bindgen::prelude::*;

const UNIVERSE_SIZE: [u32; 2] = [64, 64];
const CELL_SIZE: u32 = 10;
const LINE_THICKNESS: u32 = 1;

const fn cell_color(cell: Option<universe::Cell>) -> [u8; 4] {
    match cell {
        None => [211, 211, 211, 255],
        Some(universe::Cell::Alive) => [0, 0, 0, 255],
        Some(universe::Cell::Dying) => [255, 0, 0, 255],
        Some(universe::Cell::Dead) => [255, 255, 255, 255],
    }
}

/// # Panics
/// May panic if anything goes wrong with the DOM
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let p_alive_slider = Rc::new(Input::get("p_alive").unwrap());
    let universe = Rc::new(RefCell::new(Universe::new(
        UNIVERSE_SIZE,
        p_alive_slider.value_as_number() / 100.,
    )));
    let mut grid = Grid::new(
        "canvas",
        &universe.borrow(),
        CELL_SIZE,
        LINE_THICKNESS,
        cell_color,
    )
    .unwrap();

    let speed_slider = Rc::new(Input::get("speed").unwrap());
    #[allow(clippy::cast_possible_truncation)] // we control the HTML value, won't overflow
    let delay = Rc::new(Cell::new(-speed_slider.value_as_number() as i32));
    let ticker = Rc::new(Ticker::new(
        {
            let universe = universe.clone();
            universe.borrow_mut().tick();
            move || {
                grid.update(&universe.borrow())?;
                universe.borrow_mut().tick();
                Ok(())
            }
        },
        delay.clone(),
    ));
    speed_slider.on_change(move |this| {
        #[allow(clippy::cast_possible_truncation)]
        delay.set(-this.value_as_number() as i32);
    });

    let step_btn = Rc::new(Button::get("step").unwrap());
    step_btn.on_click({
        let ticker = ticker.clone();
        move |_| ticker.step()
    });

    let play_btn = Rc::new(Button::get("play-pause").unwrap());
    play_btn.on_click({
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

    p_alive_slider.on_change(move |this| {
        let p_alive = this.value_as_number() / 100.;
        ticker.stop();
        universe.borrow_mut().randomize(p_alive);
        ticker.step();
        play_btn.set_text_content("Play".into());
        step_btn.set_disabled(false);
        speed_slider.set_disabled(true);
    });
}
