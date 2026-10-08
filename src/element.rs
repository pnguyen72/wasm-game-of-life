use wasm_bindgen::prelude::*;
use web_sys::window;

pub mod button;
pub mod input;

pub fn get_element_by_id<T: JsCast>(id: &str) -> Option<T> {
    window()?.document()?.get_element_by_id(id)?.dyn_into().ok()
}
