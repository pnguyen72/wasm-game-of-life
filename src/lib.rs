use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Dying,
    Alive,
}

impl Cell {
    fn new_random(p_alive: f64) -> Self {
        if rand::random_bool(p_alive) {
            Self::Alive
        } else {
            Self::Dead
        }
    }

    fn toggle(&mut self) {
        *self = match self {
            Self::Dead => Self::Alive,
            Self::Dying => Self::Dead,
            Self::Alive => Self::Dying,
        };
    }
}

#[wasm_bindgen]
pub struct Board {
    cells: Vec<Cell>,
    buffer: Vec<Cell>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl Board {
    pub fn new_random(p_alive: f64) -> Self {
        let (width, height) = (64, 64);
        let cells = (0..width * height)
            .map(|_| Cell::new_random(p_alive))
            .collect::<Vec<_>>();
        let buffer = cells.clone();
        Self {
            cells,
            buffer,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        let i = self.get_index(row, col);
        self.cells[i].toggle();
    }

    pub fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.get_index(row, col);
                self.buffer[i] = match self.cells[i] {
                    Cell::Alive => Cell::Dying,
                    Cell::Dying => Cell::Dead,
                    Cell::Dead => match self.count_live_neighbors(row, col) {
                        2 => Cell::Alive,
                        _ => Cell::Dead,
                    },
                };
            }
        }
        std::mem::swap(&mut self.cells, &mut self.buffer);
    }

    fn count_live_neighbors(&self, row: usize, col: usize) -> usize {
        NEIGHBOR_OFFSETS
            .iter()
            .filter(|[i, j]| {
                let n_row = row.wrapping_add_signed(*i);
                let n_col = col.wrapping_add_signed(*j);
                let i = self.get_index(n_row, n_col);
                self.cells[i] == Cell::Alive
            })
            .count()
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        let row = row % self.height;
        let col = col % self.width;
        row * self.width + col
    }
}

static NEIGHBOR_OFFSETS: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];
