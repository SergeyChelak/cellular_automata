use std::collections::HashSet;

use rand::prelude::*;

pub type Matrix = Vec<Vec<u8>>;

#[derive(Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Default, Copy, Clone)]
pub struct Dimension {
    pub rows: usize,
    pub cols: usize,
}

pub fn size(matrix: &Matrix) -> Option<Dimension> {
    let rows = matrix.len();
    if rows == 0 {
        return None;
    }
    let Some(cols) = matrix.first().map(|row| row.len()) else {
        return None;
    };
    Some(Dimension { rows, cols })
}

pub fn noise_matrix(matrix: &mut Matrix, noise_density: u8, val_on: u8, val_off: u8) {
    let mut rng = thread_rng();
    for row in matrix.iter_mut() {
        for elem in row {
            let val: u8 = rng.gen_range(1..=100);
            *elem = if val < noise_density { val_on } else { val_off }
        }
    }
}

pub fn fill_borders(matrix: &mut Matrix, fill: u8) {
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

pub fn moore_neighborhood(input: &Matrix, val_on: u8, val_off: u8) -> Option<Matrix> {
    let Some(size) = size(input) else {
        return None;
    };
    let mut output = input.clone();
    for i in 1..size.rows - 1 {
        for j in 1..size.cols - 1 {
            let mut wall_count = 0;
            for adj_i in i - 1..=i + 1 {
                for adj_j in j - 1..=j + 1 {
                    if adj_i == i && adj_j == j {
                        continue;
                    }
                    if input[adj_i][adj_j] == val_on {
                        wall_count += 1;
                    }
                }
            }
            output[i][j] = if wall_count > 4 { val_on } else { val_off };
        }
    }
    Some(output)
}

pub fn fill_ratio(input: &Matrix, value: u8) -> f32 {
    let Some(size) = size(input) else {
        return 0.0;
    };
    let total = (size.rows * size.cols) as f32;
    let fill = input
        .iter()
        .map(|v| v.iter().filter(|&x| *x == value).count())
        .sum::<usize>();
    fill as f32 / total
}

pub fn contours(matrix: &Matrix, value: u8) -> HashSet<Position> {
    let mut positions = HashSet::<Position>::new();
    let rows = matrix.len();
    for (i, row) in matrix.iter().enumerate() {
        let cols = row.len();
        for (j, val) in row.iter().enumerate() {
            if *val != value {
                continue;
            }
            let mut adj = Vec::with_capacity(4);
            if i > 0 {
                adj.push(matrix[i - 1][j]);
            }
            if i < rows - 1 {
                adj.push(matrix[i + 1][j]);
            }
            if j > 0 {
                adj.push(matrix[i][j - 1]);
            }
            if j < cols - 1 {
                adj.push(matrix[i][j + 1]);
            }
            if adj.iter().all(|x| *x == *val) {
                continue;
            }
            positions.insert(Position { row: i, col: j });
        }
    }
    positions
}
