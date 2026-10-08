use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use wasm_bindgen::prelude::*;
use web_sys::{console, window};

pub struct Ticker {
    start: Box<dyn Fn()>,
    step: Rc<RefCell<dyn FnMut() -> Result<(), JsValue>>>,
    running: Rc<Cell<bool>>,
}

impl Ticker {
    // callback returns bool indicating whether the loop should continue
    pub fn new(
        run_fn: impl FnMut() -> Result<(), JsValue> + 'static,
        delay: Rc<Cell<i32>>,
    ) -> Self {
        let running = Rc::new(Cell::new(false));

        let step = Rc::new(RefCell::new(run_fn));

        let start = Box::new({
            let running = running.clone();
            let step = step.clone();
            let f = Rc::new(RefCell::new(None));
            let g = f.clone();
            *g.borrow_mut() = Some(Closure::new({
                let delay = delay.clone();
                move || {
                    if running.get()
                        && step.borrow_mut()()
                            .map_err(|e| console::error_1(&e))
                            .is_ok()
                    {
                        set_timeout(f.borrow().as_ref().unwrap(), delay.get());
                    } else {
                        running.set(false);
                    }
                }
            }));
            move || {
                set_timeout(g.borrow().as_ref().unwrap(), delay.get());
            }
        });

        Self {
            start,
            step,
            running,
        }
    }

    /**
     * Run repeatedly until `stop` is called, or the tick function returns an error.
     * No-op if already running.
     */
    pub fn start(&self) {
        if !self.is_running() {
            self.running.set(true);
            (self.start)();
        }
    }

    /** Run once. No-op if already running. */
    pub fn step(&self) {
        if !self.is_running()
            && let Err(e) = self.step.borrow_mut()()
        {
            console::error_1(&e);
        }
    }

    /** Stop the loop. No-op if not running. */
    pub fn stop(&self) {
        self.running.set(false);
    }

    pub fn is_running(&self) -> bool {
        self.running.get()
    }
}

fn set_timeout(cb: &Closure<dyn FnMut()>, timeout: i32) -> Option<i32> {
    window()?
        .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), timeout)
        .ok()
}
