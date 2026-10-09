use std::rc::Weak;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use wasm_bindgen::prelude::*;
use web_sys::window;

pub struct Ticker {
    tick: RefCell<Box<dyn FnMut() -> bool>>,
    delay: Cell<i32>,
    running: Cell<bool>,
    timer: Closure<dyn FnMut()>,
}

impl Ticker {
    pub fn new(tick: impl FnMut() -> bool + 'static, delay: i32) -> Rc<Self> {
        Rc::new_cyclic(|weak: &Weak<Self>| Self {
            tick: RefCell::new(Box::new(tick)),
            delay: Cell::new(delay),
            running: Cell::new(false),
            timer: Closure::new({
                let weak = weak.clone();
                move || {
                    if let Some(ticker) = weak.upgrade().filter(|t| t.is_running()) {
                        let mut tick = ticker.tick.borrow_mut();
                        if tick() {
                            set_timeout(&ticker.timer, ticker.delay.get());
                        }
                    }
                }
            }),
        })
    }

    pub fn start(&self) {
        if !self.running.get() {
            self.running.set(true);
            set_timeout(&self.timer, self.delay.get());
        }
    }

    pub fn step(&self) {
        if !self.running.get() {
            let mut tick = self.tick.borrow_mut();
            tick();
        }
    }

    pub fn stop(&self) {
        self.running.set(false);
    }

    pub fn set_delay(&self, delay: i32) {
        self.delay.set(delay);
    }

    pub const fn is_running(&self) -> bool {
        self.running.get()
    }
}

fn set_timeout(cb: &Closure<dyn FnMut()>, timeout: i32) -> Option<i32> {
    window()?
        .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), timeout)
        .ok()
}
