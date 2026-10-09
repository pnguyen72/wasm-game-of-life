use crate::error::{JsOption, JsResult};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;

mod app;
mod element;
mod error;
mod grid;
mod ticker;
mod universe;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    window()
        .and_then(|w| w.document())
        .and_then(|doc| doc.body())
        .js_expect("document.body should exist")
        // hide the loading spinner
        .and_then(|body| body.set_attribute("data-wasm-loaded", "true").js_ok())
        .and_then(|()| app::main());
}
