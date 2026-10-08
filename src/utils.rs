use wasm_bindgen::prelude::*;
use web_sys::window;

pub fn get_element_by_id<T: JsCast>(id: &str) -> Option<T> {
    window()?.document()?.get_element_by_id(id)?.dyn_into().ok()
}

pub fn set_interval(cb: &Closure<dyn FnMut()>, timeout: i32) -> Option<i32> {
    window()?
        .set_interval_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            timeout,
        )
        .ok()
}

pub fn clear_interval(handle: i32) -> Option<()> {
    window().map(|w| w.clear_interval_with_handle(handle))
}
