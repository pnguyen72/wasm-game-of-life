mod button;
mod grid;
mod ticker;
mod universe;
mod utils;

use crate::{
    button::Button,
    grid::Grid,
    ticker::Ticker,
    universe::{Cell, Universe},
};
use std::rc::Rc;
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

    let mut universe = Universe::new(UNIVERSE_SIZE, 0.02);
    let mut grid = Grid::new("canvas", &universe, CELL_SIZE, LINE_THICKNESS, cell_color).unwrap();

    let ticker = Rc::new(Ticker::new(
        move || {
            universe.tick();
            grid.update(&universe)
        },
        Rc::new(20.into()),
    ));

    let step_btn = Rc::new(Button::get("step").unwrap());
    let play_btn = Button::get("play-pause").unwrap();
    step_btn.on_click({
        let ticker = ticker.clone();
        move |_| ticker.step()
    });
    play_btn.on_click({
        let play_btn = play_btn.clone();
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
}
