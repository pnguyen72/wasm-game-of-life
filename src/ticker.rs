use crate::error::JsResult;
use std::{
    cell::{Cell, RefCell},
    rc::{Rc, Weak},
};
use wasm_bindgen::prelude::*;
use web_sys::window;

pub struct Ticker {
    tick: RefCell<Box<dyn FnMut() -> bool>>,
    timer: Closure<dyn FnMut()>,
    delay: Cell<i32>,
    timer_id: Cell<Option<i32>>,
}

impl Ticker {
    pub fn new(tick: impl FnMut() -> bool + 'static, delay: i32) -> Rc<Self> {
        Rc::new_cyclic(|weak: &Weak<Self>| Self {
            tick: RefCell::new(Box::new(tick)),
            delay: Cell::new(delay),
            timer_id: Cell::new(None),
            timer: Closure::new({
                let weak = weak.clone();
                move || {
                    if let Some(ticker) = weak.upgrade() {
                        let success = ticker.step();
                        if !success {
                            ticker.stop();
                        }
                    }
                }
            }),
        })
    }

    pub fn start(&self) {
        if !self.is_running() {
            let delay = self.delay.get();
            let timer_id = set_interval(&self.timer, delay);
            self.timer_id.set(timer_id);
        }
    }

    pub fn step(&self) -> bool {
        self.tick.borrow_mut()()
    }

    pub fn stop(&self) {
        self.timer_id.take().and_then(clear_interval);
    }

    pub const fn is_running(&self) -> bool {
        self.timer_id.get().is_some()
    }

    pub fn set_delay(&self, delay: i32) {
        self.delay.set(delay);
        if self.is_running() {
            self.stop();
            self.start();
        }
    }
}

fn set_interval(cb: &Closure<dyn FnMut()>, timeout: i32) -> Option<i32> {
    window()?
        .set_interval_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            timeout,
        )
        .js_ok()
}

fn clear_interval(handle: i32) -> Option<()> {
    window().map(|w| w.clear_interval_with_handle(handle))
}
