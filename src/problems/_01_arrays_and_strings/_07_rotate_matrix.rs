pub trait RotateMatrix {
    /// Given an image represented by an NxN matrix, where each pixel in the
    /// image is 4 bytes, write a method to rotate the image by 90 degrees. Can
    /// you do this in place?
    fn rotate_matrix(image: &mut Vec<Pixel>, n: usize);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pixel([u8; 4]);

struct Solution;

impl RotateMatrix for Solution {
    fn rotate_matrix(image: &mut Vec<Pixel>, n: usize) {
        // Replace with your solution
        use crate::solutions::_01_arrays_and_strings::_07_rotate_matrix as solutions;
        solutions::Solution::rotate_matrix(image, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_3x3_image() {
        #[rustfmt::skip]
        let mut input = monochrome_image(&[
            1, 2, 3, 
            4, 5, 6,
            7, 8, 9,
        ]);
        #[rustfmt::skip]
        let expected = monochrome_image(&[
            7, 4, 1,
            8, 5, 2,
            9, 6, 3,
        ]);
        Solution::rotate_matrix(&mut input, 3);
        assert_eq!(input, expected);
    }

    #[test]
    fn rotates_4x4_image() {
        #[rustfmt::skip]
        let mut input = monochrome_image(&[
             1,  2,  3,  4,
             5,  6,  7,  8,
             9, 10, 11, 12,
            13, 14, 15, 16,
        ]);
        #[rustfmt::skip]
        let expected = monochrome_image(&[
            13,  9, 5, 1,
            14, 10, 6, 2,
            15, 11, 7, 3,
            16, 12, 8, 4,
        ]);
        Solution::rotate_matrix(&mut input, 4);
        assert_eq!(input, expected);
    }

    fn monochrome_image(values: &[u8]) -> Vec<Pixel> {
        values.into_iter().map(|&v| Pixel([v, v, v, v])).collect()
    }
}
