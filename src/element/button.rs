use crate::element::get_element_by_id;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlButtonElement, console};

pub struct Button(HtmlButtonElement);

impl Deref for Button {
    type Target = HtmlButtonElement;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Button {
    pub fn get(element_id: &str) -> Option<Self> {
        get_element_by_id(element_id).map(Self)
    }

    pub fn on_click(&self, mut callback: impl FnMut(HtmlButtonElement) + 'static) {
        let handler = Closure::new(move |e: Event| {
            if let Some(button) = e.target().and_then(|e| e.dyn_into().ok()) {
                callback(button);
            }
        });
        if let Err(e) =
            self.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
        {
            console::error_1(&e);
        }
        handler.forget();
    }
}
