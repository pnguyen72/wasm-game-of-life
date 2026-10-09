use crate::{
    element::Element,
    error::{JsOption, JsResult},
    universe::{LifeState, Universe},
};
use wasm_bindgen::{Clamped, prelude::*};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

type ColorFn = fn(Option<LifeState>) -> [u8; 4]; // none = grid lines

pub struct Grid {
    width: u32,
    height: u32,
    cell_size: u32,
    line_thickness: u32,
    color: ColorFn,
    ctx: CanvasRenderingContext2d,
    buffer: Vec<u8>,
}

type Canvas = Element<HtmlCanvasElement>;

impl Grid {
    pub fn init(
        canvas_element_id: &str,
        universe: &Universe,
        line_thickness: u32,
        color: ColorFn,
    ) -> Option<Self> {
        let canvas = Canvas::get_by_id(canvas_element_id)?;

        let cell_size = best_cell_size(&canvas, universe, line_thickness)?;
        let width = cell_size * universe.width + line_thickness * (universe.width + 1);
        let height = cell_size * universe.height + line_thickness * (universe.height + 1);
        canvas.set_width(width);
        canvas.set_height(height);

        let ctx: CanvasRenderingContext2d =
            canvas.get_context("2d").js_ok()??.dyn_into().js_ok()?;
        let buffer_size = (width * height * 4) as usize;
        let buffer = vec![0; buffer_size];

        let mut grid = Self {
            width,
            height,
            cell_size,
            line_thickness,
            color,
            ctx,
            buffer,
        };
        grid.draw_grid_lines();
        grid.update(universe).js_ok()?;
        Some(grid)
    }

    pub fn update(&mut self, universe: &Universe) -> Result<(), JsValue> {
        self.draw_universe(universe);
        self.render()
    }

    fn draw_universe(&mut self, universe: &Universe) {
        let fill_size = [self.cell_size, self.cell_size];

        for row in 0..universe.height {
            for col in 0..universe.width {
                let cell = universe.get_cell(row, col);
                let cell_color = (self.color)(Some(cell));
                let x = self.cell_size * col + self.line_thickness * (col + 1);
                let y = self.cell_size * row + self.line_thickness * (row + 1);
                self.fill_rect(cell_color, [x, y], fill_size);
            }
        }
    }

    fn draw_grid_lines(&mut self) {
        let grid_color = (self.color)(None);
        let step = self.cell_size as usize + self.line_thickness as usize;
        // vertical
        for x in (0..self.width).step_by(step) {
            self.fill_rect(grid_color, [x, 0], [self.line_thickness, self.height]);
        }
        // horizontal
        for y in (0..self.height).step_by(step) {
            self.fill_rect(grid_color, [0, y], [self.width, self.line_thickness]);
        }
    }

    fn fill_rect(&mut self, color: [u8; 4], [x, y]: [u32; 2], [w, h]: [u32; 2]) {
        for y in y..y + h {
            for x in x..x + w {
                let idx = ((y * self.width + x) * 4) as usize;
                self.buffer[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }

    fn render(&self) -> Result<(), JsValue> {
        ImageData::new_with_u8_clamped_array(Clamped(&self.buffer), self.width)
            .and_then(|data| self.ctx.put_image_data(&data, 0., 0.))
    }
}

/// Find the largest cell size so that the canvas still fits the screen
fn best_cell_size(canvas: &Canvas, universe: &Universe, line_thickness: u32) -> Option<u32> {
    let container = canvas
        .parent_element()
        .js_expect("canvas container should exist")?;

    let w_container = i64::from(container.client_width());
    let h_container = i64::from(container.client_height());

    let w_universe = i64::from(universe.width);
    let h_universe = i64::from(universe.height);
    let line_thickness = i64::from(line_thickness);

    let width = (w_container - line_thickness * (w_universe + 1)) / w_universe;
    let height = (h_container - line_thickness * (h_universe + 1)) / h_universe;

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    Some(width.min(height).max(1) as u32)
}
