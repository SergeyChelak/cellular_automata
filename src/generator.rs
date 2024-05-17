use std::collections::HashSet;

use crate::matrix::{
    contours, fill_borders, moore_neighborhood, noise_matrix, regions, Matrix, Position,
};

const MATRIX_ROWS: usize = 70;
const MATRIX_COLS: usize = 70;
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
            noise_density: 58,
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
        self.regions = regions(&self.matrix, TILE_WALL);
        for region in self.regions.iter() {
            if region.len() > REGION_THRESHOLD {
                continue;
            }
            for pos in region.iter() {
                self.matrix[pos.row][pos.col] = TILE_FLOOR;
            }
        }
        self.contours = contours(&self.matrix, TILE_WALL);
        self.regions = regions(&self.matrix, TILE_WALL);
    }

    pub fn full_cycle(&mut self) {
        self.regenerate();
        let regions = regions(&self.matrix, TILE_FLOOR);
        let max = regions.iter().map(|x| x.len()).max().unwrap_or_default();
        for region in regions.iter() {
            if region.len() == max {
                continue;
            }
            for pos in region.iter() {
                self.matrix[pos.row][pos.col] = TILE_WALL;
            }
        }
        self.filter();
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
