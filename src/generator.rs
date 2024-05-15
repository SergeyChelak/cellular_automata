use rand::prelude::*;

const MATRIX_ROWS: usize = 200;
const MATRIX_COLS: usize = 200;

const TILE_FLOOR: u8 = 0;
const TILE_WALL: u8 = 1;

type Matrix = Vec<Vec<u8>>;

pub struct Generator {
    noise_density: u8,
    iterations: usize,
    matrix: Matrix,
}

impl Generator {
    pub fn new() -> Self {
        let matrix = vec![vec![0; MATRIX_COLS]; MATRIX_ROWS];
        Self {
            noise_density: 50,
            iterations: 0,
            matrix,
        }
    }

    fn generate(&mut self) {
        self.matrix = moore_neighborhood(&self.matrix)
    }

    pub fn noise_density(&self) -> u8 {
        self.noise_density
    }

    pub fn iterations(&self) -> usize {
        self.iterations
    }

    pub fn get_matrix(&self) -> &Vec<Vec<u8>> {
        &self.matrix
    }

    pub fn regenerate(&mut self) {
        noise_matrix(&mut self.matrix, self.noise_density);
        for _ in 0..self.iterations {
            self.generate();
        }
    }

    pub fn next_iteration(&mut self) {
        self.increase_iterations();
        self.generate();
    }

    pub fn increase_noise_density(&mut self) {
        if self.noise_density < 100 {
            self.noise_density += 1;
        }
        self.regenerate();
    }

    pub fn decrease_noise_density(&mut self) {
        self.noise_density = self.noise_density.saturating_sub(1);
        self.regenerate();
    }

    pub fn increase_iterations(&mut self) {
        self.iterations = self.iterations.saturating_add(1);
        self.regenerate();
    }

    pub fn decrease_iterations(&mut self) {
        self.iterations = self.iterations.saturating_sub(1);
        self.regenerate();
    }
}

fn noise_matrix(matrix: &mut Matrix, noise_density: u8) {
    let mut rng = thread_rng();
    for row in matrix.iter_mut() {
        for elem in row {
            let val: u8 = rng.gen_range(1..=100);
            *elem = if val > noise_density {
                TILE_FLOOR
            } else {
                TILE_WALL
            }
        }
    }

    let fill = TILE_WALL;
    for row in matrix.iter_mut() {
        *row.first_mut().unwrap() = fill;
        *row.last_mut().unwrap() = fill;
    }

    matrix
        .first_mut()
        .unwrap()
        .iter_mut()
        .for_each(|x| *x = fill);

    matrix
        .last_mut()
        .unwrap()
        .iter_mut()
        .for_each(|x| *x = fill);
}

fn moore_neighborhood(input: &Matrix) -> Matrix {
    let row_count = input.len();
    let col_count = input.first().unwrap().len();
    let mut output = input.clone();
    for i in 1..row_count - 1 {
        for j in 1..col_count - 1 {
            let mut wall_count = 0;
            for adj_i in i - 1..=i + 1 {
                for adj_j in j - 1..=j + 1 {
                    if adj_i == i && adj_j == j {
                        continue;
                    }
                    if input[adj_i][adj_j] == TILE_WALL {
                        wall_count += 1;
                    }
                }
            }
            output[i][j] = if wall_count > 4 {
                TILE_WALL
            } else {
                TILE_FLOOR
            };
        }
    }
    output
}
