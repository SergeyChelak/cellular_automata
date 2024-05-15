use std::collections::HashSet;

use crate::matrix::{
    contours, fill_borders, fill_ratio, moore_neighborhood, noise_matrix, regions, Matrix, Position,
};

const MATRIX_ROWS: usize = 100;
const MATRIX_COLS: usize = 100;
const REGION_THRESHOLD: usize = 3;

const TILE_FLOOR: u8 = 0;
const TILE_WALL: u8 = 1;

pub struct Generator {
    noise_density: u8,
    iterations: usize,
    matrix: Matrix,
    contours: HashSet<Position>,
    regions: Vec<HashSet<Position>>,
}

impl Generator {
    pub fn new() -> Self {
        let matrix = vec![vec![0; MATRIX_COLS]; MATRIX_ROWS];
        Self {
            noise_density: 52,
            iterations: 3,
            matrix,
            contours: Default::default(),
            regions: Default::default(),
        }
    }

    fn generate(&mut self) {
        let Some(matrix) = moore_neighborhood(&self.matrix, TILE_WALL, TILE_FLOOR) else {
            return;
        };
        self.matrix = matrix;
        self.regions = regions(&self.matrix, TILE_WALL);
        self.contours = contours(&self.matrix, TILE_WALL);
    }

    pub fn filter(&mut self) {
        let regions = regions(&mut self.matrix, TILE_WALL);
        for region in regions.iter() {
            if region.len() > REGION_THRESHOLD {
                continue;
            }
            for pos in region.iter() {
                self.matrix[pos.row][pos.col] = TILE_FLOOR;
            }
        }
        self.contours = contours(&self.matrix, TILE_WALL);
    }

    fn print_fill_rate(&self) {
        println!("fill ratio: {:.2}", fill_ratio(&self.matrix, TILE_WALL));
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

    pub fn get_contours(&self) -> &HashSet<Position> {
        &self.contours
    }

    pub fn region_id(&self, position: &Position) -> Option<usize> {
        for (id, set) in self.regions.iter().enumerate() {
            if set.contains(position) {
                return Some(id);
            }
        }
        None
    }

    pub fn regenerate(&mut self) {
        noise_matrix(&mut self.matrix, self.noise_density, TILE_WALL, TILE_FLOOR);
        fill_borders(&mut self.matrix, TILE_WALL);
        for _ in 0..self.iterations {
            self.generate();
        }
        self.print_fill_rate();
    }

    pub fn next_iteration(&mut self) {
        self.increase_iterations();
        self.generate();
        self.print_fill_rate();
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
