use super::matrix::Matrix;

pub trait RotateMatrix {
    /// Given an NxN matrix write a method to rotate it by 90 degrees.
    fn rotate_matrix(matrix: &mut Matrix);
}

struct Solution;

impl RotateMatrix for Solution {
    fn rotate_matrix(matrix: &mut Matrix) {
        // Replace with your solution
        use crate::solutions::_01_arrays_and_strings::_07_rotate_matrix as solutions;
        solutions::Solution::rotate_matrix(matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_3x3() {
        #[rustfmt::skip]
        let mut input = Matrix {
            m: 3,
            n: 3,
            cells: vec![
                1, 2, 3, 
                4, 5, 6,
                7, 8, 9,
            ],
        };

        #[rustfmt::skip]
        let expected = Matrix {
            m: 3,
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
    fn rotates_4x4() {
        #[rustfmt::skip]
        let mut input = Matrix {
            m: 4,
            n: 4,
            cells: vec![
                 1,  2,  3,  4,
                 5,  6,  7,  8,
                 9, 10, 11, 12,
                13, 14, 15, 16,
            ],
        };

        #[rustfmt::skip]
        let expected = Matrix {
            m: 4,
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
