use rand::prelude::*;

const MATRIX_ROWS: usize = 30;
const MATRIX_COLS: usize = 30;

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
        //
    }

    pub fn get_matrix(&self) -> &Vec<Vec<u8>> {
        &self.matrix
    }

    pub fn regenerate(&mut self) {
        noise_matrix(&mut self.matrix, self.noise_density);
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
            let val: u8 = rng.gen_range(0..100);
            *elem = if val > noise_density { 0 } else { 1 }
        }
    }
}
