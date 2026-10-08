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

    pub fn on_change(&self, mut callback: impl FnMut(HtmlInputElement) + 'static) {
        let handler = Closure::new(move |e: Event| {
            if let Some(input) = e.target().and_then(|e| e.dyn_into().ok()) {
                callback(input);
            }
        });
        if let Err(e) =
            self.add_event_listener_with_callback("input", handler.as_ref().unchecked_ref())
        {
            console::error_1(&e);
        }
        handler.forget();
    }
}
