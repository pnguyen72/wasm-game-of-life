use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;

use crate::error::{JsOption, JsResult};

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

    window()
        .and_then(|w| w.document())
        .and_then(|doc| doc.body())
        .js_expect("document.body not found")
        .map(|body| body.set_attribute("data-wasm-loaded", "true").js_ok());
}
