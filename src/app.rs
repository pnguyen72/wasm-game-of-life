use crate::{
    element::Element,
    error::JsResult,
    grid::Grid,
    ticker::Ticker,
    universe::{LifeState, Universe},
};
use std::{cell::RefCell, rc::Rc};
use web_sys::{HtmlButtonElement, HtmlInputElement};

const UNIVERSE_SIZE: [u32; 2] = [64, 64];
const LINE_THICKNESS: u32 = 1;

const fn cell_color(cell: Option<LifeState>) -> [u8; 4] {
    match cell {
        None => [242, 233, 225, 255],
        Some(LifeState::Healthy) => [70, 66, 97, 255],
        Some(LifeState::Dying) => [235, 111, 146, 255],
        Some(LifeState::Dead) => [0, 0, 0, 0],
    }
}

pub fn main() -> Option<()> {
    // Initialize
    let play_pause_btn = Button::get_by_id("play/pause")?;
    let step_restart_btn = Button::get_by_id("step/restart")?;

    let p_alive_slider = Input::get_by_id("p_alive")?;
    let speed_slider = Input::get_by_id("speed")?;

    let universe = Rc::new(RefCell::new(Universe::new(
        UNIVERSE_SIZE,
        p_alive_slider.get_p_alive(),
    )));
    let mut grid = Grid::init("canvas", &universe.borrow(), LINE_THICKNESS, cell_color)?;

    // Core logic
    universe.borrow_mut().tick();
    let ticker = Ticker::new(
        {
            let universe = universe.clone();
            let play_pause_btn = play_pause_btn.clone();

            move || {
                let mut universe = universe.borrow_mut();
                let success = (grid.update(&universe).js_ok())
                    .filter(|()| universe.is_alive())
                    .map(|()| universe.tick())
                    .is_some();
                if !success {
                    play_pause_btn.set_disabled(true);
                }
                success
            }
        },
        speed_slider.get_delay(),
    );

    // Attach event handlers
    step_restart_btn.on("click", {
        let universe = universe.clone();
        let ticker = ticker.clone();
        let play_pause_btn = play_pause_btn.clone();
        let p_alive_slider = p_alive_slider.clone();

        move |_| {
            let is_running = ticker.is_running();
            if !is_running && universe.borrow().is_alive() {
                ticker.step();
                return;
            }
            universe
                .borrow_mut()
                .randomize(p_alive_slider.get_p_alive());
            if !is_running {
                ticker.start();
                play_pause_btn.set_disabled(false);
            }
        }
    });

    play_pause_btn.on("click", {
        let ticker = ticker.clone();
        let step_restart_btn = step_restart_btn.clone();

        move |this| {
            if ticker.is_running() {
                this.set_text_content("Play".into());
                step_restart_btn.set_text_content("Step".into());
                ticker.stop();
            } else {
                this.set_text_content("Pause".into());
                step_restart_btn.set_text_content("Restart".into());
                ticker.start();
            }
        }
    });

    speed_slider.on("change", {
        let ticker = ticker.clone();
        move |this| ticker.set_delay(this.get_delay())
    });

    p_alive_slider.on("input", move |this| {
        ticker.stop();
        universe.borrow_mut().randomize(this.get_p_alive());
        ticker.step();

        play_pause_btn.set_text_content("Play".into());
        step_restart_btn.set_text_content("Step".into());
        play_pause_btn.set_disabled(false);
    });

    Some(())
}

type Button = Element<HtmlButtonElement>;
type Input = Element<HtmlInputElement>;

impl Input {
    fn get_p_alive(&self) -> f64 {
        // html value is in percentage, convert it to decimal
        self.value_as_number() / 100.
    }

    #[allow(clippy::cast_possible_truncation)] // we control the HTML value, it won't overflow
    fn get_delay(&self) -> i32 {
        // html value is as speed; speed = -delay
        -self.value_as_number() as i32
    }
}
