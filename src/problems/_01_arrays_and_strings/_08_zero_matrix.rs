use super::Matrix;

pub trait ZeroMatrix {
    /// Write an algorithm such that if an elment in an MxN matrix is zero, its
    /// entire row and column are set to zero.
    fn zero_matrix(matrix: &Matrix) -> Matrix;
}

struct Solution;

impl ZeroMatrix for Solution {
    fn zero_matrix(matrix: &Matrix) -> Matrix {
        // Replace with your solution
        use crate::solutions::_01_arrays_and_strings::_08_zero_matrix as solutions;
        solutions::Solution::zero_matrix(matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_zeroes_untouched() {
        #[rustfmt::skip]
        let input = Matrix {
            m: 3,
            n: 3,
            cells: vec![
                1, 2, 3,
                4, 5, 6,
                7, 8, 9,
            ],
        };
        let expected = input.clone();
        assert_eq!(Solution::zero_matrix(&input), expected);
    }

    #[test]
    fn zeroes_single_row_and_column() {
        #[rustfmt::skip]
        let input = Matrix {
            m: 3,
            n: 3,
            cells: vec![
                1, 2, 3,
                4, 0, 6,
                7, 8, 9,
            ],
        };

        #[rustfmt::skip]
        let expected = Matrix {
            m: 3,
            n: 3,
            cells: vec![
                1, 0, 3,
                0, 0, 0,
                7, 0, 9,
            ],
        };

        assert_eq!(Solution::zero_matrix(&input), expected);
    }

    #[test]
    fn zeroes_multiple_rows_and_columns() {
        #[rustfmt::skip]
        let input = Matrix {
            m: 6,
            n: 4,
            cells: vec![
                1, 1, 1, 1, 
                1, 1, 1, 0, 
                1, 1, 1, 1, 
                1, 1, 1, 1, 
                1, 0, 1, 1, 
                1, 1, 1, 1, 
            ]
        };

        #[rustfmt::skip]
        let expected = Matrix {
            m: 6,
            n: 4,
            cells: vec![
                1, 0, 1, 0, 
                0, 0, 0, 0, 
                1, 0, 1, 0, 
                1, 0, 1, 0, 
                0, 0, 0, 0, 
                1, 0, 1, 0, 
            ]
        };

        assert_eq!(Solution::zero_matrix(&input), expected);
    }
}
