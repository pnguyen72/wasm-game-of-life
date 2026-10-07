use std::{cell::RefCell, rc::Rc};

use crate::{
    board::Board,
    universe::{Cell, Universe},
};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, console, window};

extern crate console_error_panic_hook;

mod board;
mod universe;

const UNIVERSE_SIZE: [u32; 2] = [64, 64];
const CELL_SIZE: u32 = 12;
const GRID_THICKNESS: u32 = 1;

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

    let mut universe = Universe::new(UNIVERSE_SIZE, 0.5);
    let mut board = get_element_by_id::<HtmlCanvasElement>("canvas")
        .and_then(|canvas| {
            Board::new(
                UNIVERSE_SIZE,
                CELL_SIZE,
                GRID_THICKNESS,
                cell_color,
                &canvas,
            )
        })
        .expect("Unable to initialize canvas");

    render_loop(
        move || {
            board
                .update(&universe)
                .map(|()| universe.tick())
                .map_err(|e| console::error_1(&e))
                .is_ok()
        },
        16,
    );
}

fn get_element_by_id<T: JsCast>(id: &str) -> Option<T> {
    window()?.document()?.get_element_by_id(id)?.dyn_into().ok()
}

/// Callback: return true to keep going, false to stop
fn render_loop(mut cb: impl FnMut() -> bool + 'static, delay: i32) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        if cb() {
            set_timeout(f.borrow().as_ref().unwrap(), delay);
        }
    }));
    set_timeout(g.borrow().as_ref().unwrap(), delay);
}

fn set_timeout(cb: &Closure<dyn FnMut()>, timeout: i32) -> Option<i32> {
    window()?
        .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), timeout)
        .ok()
}
