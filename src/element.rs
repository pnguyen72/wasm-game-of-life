use crate::error::JsResult;
use std::fmt::Debug;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::{Event, EventTarget};

#[derive(Clone)]
pub struct Element<T: JsCast>(T);

impl<T: JsCast> Deref for Element<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: JsCast + AsRef<EventTarget> + Debug> Element<T> {
    pub fn get_by_id(id: &str) -> Option<Self> {
        let element = window()?.document()?.get_element_by_id(id)?;
        Self::from(element)
    }

    pub fn on(&self, event: &str, mut callback: impl FnMut(&Self) + 'static) {
        let handler = Closure::new(move |e: Event| {
            if let Some(target) = e.target().and_then(Self::from) {
                callback(&target);
            }
        });
        let el: &EventTarget = self.0.as_ref();
        if el
            .add_event_listener_with_callback(event, handler.as_ref().unchecked_ref())
            .js_ok()
            .is_some()
        {
            handler.forget(); // keep the closure alive for JS
        }
    }

    fn from<E: JsCast + AsRef<EventTarget> + Debug>(e: E) -> Option<Self> {
        e.dyn_into().js_ok().map(Self)
    }
}
