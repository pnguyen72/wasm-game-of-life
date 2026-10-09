use wasm_bindgen::prelude::wasm_bindgen;

mod app;
mod element;
mod error;
mod grid;
mod ticker;
mod universe;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    app::main();
}
