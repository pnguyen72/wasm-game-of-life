use crate::element::get_element_by_id;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlInputElement, console};

pub struct Input(HtmlInputElement);

impl Deref for Input {
    type Target = HtmlInputElement;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Input {
    pub fn get(element_id: &str) -> Option<Self> {
        get_element_by_id(element_id).map(Self)
    }

    pub fn on_change(&self, cb: impl FnMut(Event) + 'static) {
        let cb = Closure::new(cb);
        if let Err(e) = self.add_event_listener_with_callback("input", cb.as_ref().unchecked_ref())
        {
            console::error_1(&e);
        }
        cb.forget();
    }
}
