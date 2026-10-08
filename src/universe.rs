#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Dying,
    Alive,
}

impl Cell {
    fn new(p_alive: f64) -> Self {
        if rand::random_bool(p_alive) {
            Self::Alive
        } else {
            Self::Dead
        }
    }
}

pub struct Universe {
    pub width: u32,
    pub height: u32,
    cells: Vec<Cell>,
    buffer: Vec<Cell>,
}

impl Universe {
    pub fn new([width, height]: [u32; 2], p_alive: f64) -> Self {
        let cells = (0..width * height)
            .map(|_| Cell::new(p_alive))
            .collect::<Vec<_>>();
        let buffer = cells.clone();
        Self {
            width,
            height,
            cells,
            buffer,
        }
    }

    pub fn get_cell(&self, row: u32, col: u32) -> Cell {
        self.cells[self.get_index(row, col)]
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

    fn count_live_neighbors(&self, row: u32, col: u32) -> usize {
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

    const fn get_index(&self, row: u32, col: u32) -> usize {
        let row = row % self.height;
        let col = col % self.width;
        (row as usize) * (self.width as usize) + (col as usize)
    }
}

const NEIGHBOR_OFFSETS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];
