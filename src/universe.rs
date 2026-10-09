#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LifeState {
    Dead,
    Dying,
    Healthy,
}

impl LifeState {
    fn new(p_alive: f64) -> Self {
        if rand::random_bool(p_alive) {
            Self::Healthy
        } else {
            Self::Dead
        }
    }

    pub fn randomize(&mut self, p_alive: f64) {
        *self = Self::new(p_alive);
    }
}

pub struct Universe {
    pub width: u32,
    pub height: u32,
    cells: Vec<LifeState>,
    buffer: Vec<LifeState>,
}

impl Universe {
    pub fn new([width, height]: [u32; 2], p_alive: f64) -> Self {
        let cells = (0..width * height)
            .map(|_| LifeState::new(p_alive))
            .collect::<Vec<_>>();
        let buffer = cells.clone();
        Self {
            width,
            height,
            cells,
            buffer,
        }
    }

    pub fn randomize(&mut self, p_alive: f64) {
        self.cells.iter_mut().for_each(|c| c.randomize(p_alive));
    }

    pub fn get_cell(&self, row: u32, col: u32) -> LifeState {
        self.cells[self.get_index(row, col)]
    }

    pub fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.get_index(row, col);
                self.buffer[i] = match self.cells[i] {
                    LifeState::Healthy => LifeState::Dying,
                    LifeState::Dying => LifeState::Dead,
                    LifeState::Dead => match self.count_live_neighbors(row, col) {
                        2 => LifeState::Healthy,
                        _ => LifeState::Dead,
                    },
                };
            }
        }
        std::mem::swap(&mut self.cells, &mut self.buffer);
    }

    pub fn is_alive(&self) -> bool {
        self.cells.iter().any(|c| *c != LifeState::Dead)
    }

    fn count_live_neighbors(&self, row: u32, col: u32) -> usize {
        NEIGHBOR_OFFSETS
            .iter()
            .filter(|[i, j]| {
                let n_row = row.wrapping_add_signed(*i);
                let n_col = col.wrapping_add_signed(*j);
                let i = self.get_index(n_row, n_col);
                self.cells[i] == LifeState::Healthy
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
