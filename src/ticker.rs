use js_sys::Function;
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, window};

use crate::utils::{clear_interval, set_interval};

pub struct Ticker {
    tick: Closure<dyn FnMut()>,
    delay: Rc<Cell<i32>>,
    timer_id: Rc<Cell<Option<i32>>>,
    running: Cell<bool>,
}

impl Ticker {
    pub fn new(mut f: impl FnMut() -> Result<(), JsValue> + 'static, delay: Rc<Cell<i32>>) -> Self {
        let timer_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));

        let stop = {
            let timer_id = timer_id.clone();
            move || {
                if let (Some(w), Some(id)) = (window(), timer_id.take()) {
                    w.clear_interval_with_handle(id);
                }
            }
        };

        let tick = Closure::<dyn FnMut()>::new(move || {
            if let Err(e) = f() {
                console::error_1(&e);
                stop();
            }
        });

        let running = Cell::new(false);

        Self {
            tick,
            delay,
            timer_id,
            running,
        }
    }

    /**
     * Run repeatedly until `stop` is called, or the tick function returns an error.
     * No-op if already running.
     */
    pub fn start(&self) {
        if self.timer_id.get().is_some() {
            return; // already running
        }
        match set_interval(&self.tick, self.delay.get()) {
            None => console::error_1(&"setInterval failed".into()),
            id => {
                self.timer_id.set(id);
                self.running.set(true);
            }
        }
    }

    /** Run once. No-op if already running. */
    pub fn step(&self) {
        if self.timer_id.get().is_some() {
            return; // already running
        }
        let tick = self.tick.as_ref().unchecked_ref::<Function>();
        if let Err(e) = tick.call0(&JsValue::NULL) {
            console::error_1(&e);
        }
    }

    /** Stop the loop. No-op if not running. */
    pub fn stop(&self) {
        if self.timer_id.take().and_then(clear_interval).is_some() {
            self.running.set(false);
        }
    }

    pub const fn is_running(&self) -> bool {
        self.running.get()
    }
}
