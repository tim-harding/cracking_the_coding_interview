use crate::problems::_01_arrays_and_strings::{Matrix, _08_zero_matrix::ZeroMatrix};

pub struct Solution;

impl ZeroMatrix for Solution {
    fn zero_matrix(matrix: &Matrix) -> Matrix {
        // Track which columns have zeroes in indices 0..n and which rows have
        // zeroes in indices 0..m
        let mut bitset = Bitset::new(matrix.n + matrix.m);
        for y in 0..matrix.m {
            for x in 0..matrix.n {
                if matrix.get(x, y) == 0 {
                    bitset.set(x);
                    bitset.set(y + matrix.n);
                }
            }
        }

        // Copy over each matrix element unless there was a zero found in its
        // row or column.
        let mut out = Matrix::new(matrix.n, matrix.m, vec![0u8; matrix.n * matrix.m]);
        for y in 0..matrix.m {
            for x in 0..matrix.n {
                let cell = if bitset.get(x) || bitset.get(y + matrix.n) {
                    0
                } else {
                    matrix.get(x, y)
                };
                out.set(x, y, cell);
            }
        }
        out
    }
}

#[derive(Debug, Clone)]
struct Bitset(Vec<u8>);

impl Bitset {
    pub fn new(size: usize) -> Self {
        Self(vec![0u8; size / 8 + 1])
    }

    pub fn get(&self, i: usize) -> bool {
        (self.0[i / 8] >> (i % 8)) & 1 == 1
    }

    pub fn set(&mut self, i: usize) {
        self.0[i / 8] |= 1 << (i % 8);
    }
}
