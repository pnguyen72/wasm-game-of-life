mod element;
mod grid;
mod ticker;
mod universe;

use crate::{
    element::{button::Button, input::Input},
    grid::Grid,
    ticker::Ticker,
    universe::{Cell, Universe},
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

const UNIVERSE_SIZE: [u32; 2] = [64, 64];
const CELL_SIZE: u32 = 11;
const LINE_THICKNESS: u32 = 1;

const fn cell_color(cell: Option<Cell>) -> [u8; 4] {
    match cell {
        None => [211, 211, 211, 255],
        Some(Cell::Alive) => [0, 0, 0, 255],
        Some(Cell::Dying) => [255, 0, 0, 255],
        Some(Cell::Dead) => [255, 255, 255, 255],
    }
}

#[wasm_bindgen(start)]
/// # Panics
/// May panic if anything goes wrong with the DOM
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
        Rc::new(20.into()),
    ));

    let step_btn = Rc::new(Button::get("step").unwrap());
    step_btn.on_click({
        let ticker = ticker.clone();
        move |_| ticker.step()
    });

    let play_btn = Rc::new(Button::get("play-pause").unwrap());
    play_btn.on_click({
        let ticker = ticker.clone();
        let play_btn = play_btn.clone();
        let step_btn = step_btn.clone();
        move |_| {
            if ticker.is_running() {
                play_btn.set_text_content("Play".into());
                step_btn.set_disabled(false);
                ticker.stop();
            } else {
                play_btn.set_text_content("Pause".into());
                step_btn.set_disabled(true);
                ticker.start();
            }
        }
    });

    p_alive_slider.on_change({
        let p_alive_slider = p_alive_slider.clone();
        move |_| {
            let p_alive = p_alive_slider.value_as_number() / 100.;
            ticker.stop();
            universe.borrow_mut().randomize(p_alive);
            ticker.step();
            play_btn.set_text_content("Play".into());
            step_btn.set_disabled(false);
        }
    });
}
