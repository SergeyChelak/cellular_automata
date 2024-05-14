use rand::prelude::*;

const MATRIX_ROWS: usize = 100;
const MATRIX_COLS: usize = 100;

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
    }

    pub fn decrease_noise_density(&mut self) {
        self.noise_density = self.noise_density.saturating_sub(1);
    }

    pub fn increase_iterations(&mut self) {
        self.iterations = self.iterations.saturating_add(1);
    }

    pub fn decrease_iterations(&mut self) {
        self.iterations = self.iterations.saturating_sub(1);
    }
}

fn noise_matrix(matrix: &mut Matrix, noise_density: u8) {
    let mut rng = thread_rng();
    for row in matrix {
        for elem in row {
            let val: u8 = rng.gen_range(1..=100);
            *elem = if val > noise_density {
                TILE_FLOOR
            } else {
                TILE_WALL
            }
        }
    }
}

fn moore_neighborhood(input: &Matrix) -> Matrix {
    let mut output = input.clone();
    for (i, row) in input.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let row_beg = if i > 0 { i - 1 } else { 0 };
            let col_beg = if j > 0 { j - 1 } else { 0 };
            let mut wall_count = 0;
            for adj_i in row_beg..=i + 1 {
                for adj_j in col_beg..=j + 1 {
                    if adj_i == i && adj_j == j {
                        continue;
                    }
                    let Some(val) = input.get(adj_i).and_then(|x| x.get(adj_j)) else {
                        continue;
                    };
                    if *val == TILE_WALL {
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

// Noise density |Q+ 58 -A|  Iterations |W+ 5 -S|  R(egenerate)  N(ext iteration)
