use wasm_bindgen::{Clamped, prelude::*};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::{
    universe::{Cell, Universe},
    web_utils::get_element_by_id,
};

type ColorFn = fn(Option<Cell>) -> [u8; 4]; // none = grid lines

pub struct Canvas {
    width: u32,
    height: u32,
    cell_size: u32,
    grid_thickness: u32,
    color: ColorFn,
    ctx: CanvasRenderingContext2d,
    buffer: Vec<u8>,
}

impl Canvas {
    pub fn new(
        element_id: &str,
        universe: &Universe,
        cell_size: u32,
        grid_thickness: u32,
        color: ColorFn,
    ) -> Option<Self> {
        let width = cell_size * universe.width + grid_thickness * (universe.width + 1);
        let height = cell_size * universe.height + grid_thickness * (universe.height + 1);

        let canvas: HtmlCanvasElement = get_element_by_id(element_id)?;
        canvas.set_width(width);
        canvas.set_height(height);

        let ctx = canvas
            .get_context("2d")
            .ok()??
            .dyn_into::<CanvasRenderingContext2d>()
            .ok()?;

        let buffer_size = (width * height * 4) as usize;
        let buffer = vec![0; buffer_size];

        let mut board = Self {
            width,
            height,
            cell_size,
            grid_thickness,
            color,
            ctx,
            buffer,
        };

        board.draw_grid();
        board.draw_universe(universe);
        board.render().ok().and(Some(board))
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
                let x = self.cell_size * col + self.grid_thickness * (col + 1);
                let y = self.cell_size * row + self.grid_thickness * (row + 1);
                self.fill_rect(cell_color, [x, y], fill_size);
            }
        }
    }

    fn draw_grid(&mut self) {
        let grid_color = (self.color)(None);
        let step = self.cell_size as usize + self.grid_thickness as usize;
        // vertical
        for x in (0..self.width).step_by(step) {
            self.fill_rect(grid_color, [x, 0], [self.grid_thickness, self.height]);
        }
        // horizontal
        for y in (0..self.height).step_by(step) {
            self.fill_rect(grid_color, [0, y], [self.width, self.grid_thickness]);
        }
    }

    fn fill_rect(&mut self, color: [u8; 4], [x, y]: [u32; 2], [w, h]: [u32; 2]) {
        for py in y..(y + h) {
            for px in x..(x + w) {
                let idx = ((py * self.width + px) * 4) as usize;
                self.buffer[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }

    fn render(&self) -> Result<(), JsValue> {
        ImageData::new_with_u8_clamped_array(Clamped(&self.buffer), self.width)
            .and_then(|data| self.ctx.put_image_data(&data, 0., 0.))
    }
}
