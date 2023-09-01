use std::fmt::{self, Debug, Formatter};

pub trait RotateMatrix {
    /// Given an NxN matrix write a method to rotate it by 90 degrees.
    fn rotate_matrix(matrix: &mut SquareMatrix);
}

struct Solution;

impl RotateMatrix for Solution {
    fn rotate_matrix(matrix: &mut SquareMatrix) {
        // Replace with your solution
        use crate::solutions::_01_arrays_and_strings::_07_rotate_matrix as solutions;
        solutions::Solution::rotate_matrix(matrix)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SquareMatrix {
    pub cells: Vec<u8>,
    pub n: usize,
}

impl Debug for SquareMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cells: Vec<_> = self.cells.iter().map(|cell| format!("{cell}")).collect();
        let column_widths: Vec<_> = (0..self.n)
            .map(|i| {
                cells
                    .iter()
                    .skip(i)
                    .step_by(self.n)
                    .map(|cell| cell.len())
                    .max()
                    .unwrap()
            })
            .collect();

        for chunk in cells.chunks(self.n) {
            for (column, cell) in chunk.iter().enumerate() {
                write!(f, "{:>1$}", cell, column_widths[column] + 1)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_3x3_image() {
        #[rustfmt::skip]
        let mut input = SquareMatrix {
            n: 3,
            cells: vec![
                1, 2, 3, 
                4, 5, 6,
                7, 8, 9,
            ],
        };

        #[rustfmt::skip]
        let expected = SquareMatrix {
            n: 3,
            cells: vec![
                7, 4, 1,
                8, 5, 2,
                9, 6, 3,
            ],
        };

        Solution::rotate_matrix(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn rotates_4x4_image() {
        #[rustfmt::skip]
        let mut input = SquareMatrix {
            n: 4,
            cells: vec![
                 1,  2,  3,  4,
                 5,  6,  7,  8,
                 9, 10, 11, 12,
                13, 14, 15, 16,
            ],
        };

        #[rustfmt::skip]
        let expected = SquareMatrix {
            n: 4,
            cells: vec![
                13,  9, 5, 1,
                14, 10, 6, 2,
                15, 11, 7, 3,
                16, 12, 8, 4,
            ],
        };

        Solution::rotate_matrix(&mut input);
        assert_eq!(input, expected);
    }
}
