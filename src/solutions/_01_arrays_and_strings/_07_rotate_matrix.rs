use crate::problems::_01_arrays_and_strings::_07_rotate_matrix::{RotateMatrix, SquareMatrix};

pub struct Solution;

impl RotateMatrix for Solution {
    fn rotate_matrix(matrix: &mut SquareMatrix) {
        // Iterate over a triangle forming a 1/8th slice of the image
        let n = matrix.n;
        for y in 0..n / 2 {
            for x in y..n / 2 {
                let mut x = x;
                let mut y = y;
                let mut tmp = matrix.cells[y * n + x];
                for _ in 0..4 {
                    (x, y) = rotate_90(x, y, n);
                    std::mem::swap(&mut matrix.cells[y * n + x], &mut tmp);
                }
            }
        }
    }
}

fn rotate_90(x: usize, y: usize, n: usize) -> (usize, usize) {
    // let h = n / 2
    // Translate by (-h, -h) such that the center as at the origin
    // Apply the 90° rotation matrix
    // Reverse the translation
    // ┏      ┓┏   ┓   ┏ ┓   ┏   ┓
    // ┃  0 1 ┃┃x-h┃ + ┃h┃ = ┃  y┃
    // ┃ -1 0 ┃┃y-h┃   ┃h┃   ┃n-x┃
    // ┗      ┛┗   ┛   ┗ ┛   ┗   ┛
    (y, n - x - 1)
}
