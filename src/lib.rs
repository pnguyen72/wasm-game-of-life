mod canvas;
mod universe;
mod web_utils;

use crate::{canvas::Canvas, universe::Universe, web_utils::get_element_by_id};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlButtonElement, console, window};

const UNIVERSE_SIZE: [u32; 2] = [64, 64];
const CELL_SIZE: u32 = 11;
const GRID_THICKNESS: u32 = 1;

const fn cell_color(cell: Option<universe::Cell>) -> [u8; 4] {
    match cell {
        None => [211, 211, 211, 255],
        Some(universe::Cell::Alive) => [0, 0, 0, 255],
        Some(universe::Cell::Dying) => [255, 0, 0, 255],
        Some(universe::Cell::Dead) => [255, 255, 255, 255],
    }
}

#[wasm_bindgen(start)]
/// # Panics
/// May panic if anything goes wrong with the DOM
pub fn main() {
    console_error_panic_hook::set_once();

    let mut universe = Universe::new(UNIVERSE_SIZE, 0.1);
    let mut canvas = Canvas::new("canvas", &universe, CELL_SIZE, GRID_THICKNESS, cell_color)
        .expect("Unable to initialize canvas");

    let play_btn =
        Rc::new(get_element_by_id::<HtmlButtonElement>("play-pause").expect("play button"));
    let running = Rc::new(Cell::new(false));
    let run = {
        let running = running.clone();
        create_interval(
            move || {
                let is_running = running.get();
                is_running
                    && canvas
                        .update(&universe)
                        .map(|()| universe.tick())
                        .map_err(|e| console::error_1(&e))
                        .is_ok()
            },
            20, // TODO: dynamically update this
        )
    };
    {
        let run = run.clone();
        let closure = {
            let play_button = play_btn.clone();
            Closure::<dyn FnMut(_)>::new(move |_: Event| {
                let is_running = running.get();
                running.set(!is_running);
                if is_running {
                    play_button.set_text_content(Some("Play"));
                } else {
                    run();
                    play_button.set_text_content(Some("Pause"));
                }
            })
        };
        play_btn
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
    // run();
}

// https://wasm-bindgen.github.io/wasm-bindgen/examples/request-animation-frame.html
fn create_interval(mut cb: impl FnMut() -> bool + 'static, delay: i32) -> Rc<impl Fn()> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        // callback returns boolean for whether to continue the loop
        if cb() {
            set_timeout(f.borrow().as_ref().unwrap(), delay);
        }
    }));
    Rc::new(move || {
        set_timeout(g.borrow().as_ref().unwrap(), delay);
    })
}

fn set_timeout(cb: &Closure<dyn FnMut()>, timeout: i32) -> Option<i32> {
    window()?
        .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), timeout)
        .ok()
}
