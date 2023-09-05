pub trait Conversion {
    /// Returns the number of bits you would need to flip to convert a into b
    fn conversion(a: u64, b: u64) -> u8;
}

struct Solution;

impl Conversion for Solution {
    fn conversion(a: u64, b: u64) -> u8 {
        // Your solution here
        use crate::solutions::_05_bit_manipulation::_06_conversion as solutions;
        solutions::Solution::conversion(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_number_of_bit_flips() {
        assert_eq!(Solution::conversion(0, 0), 0);
        assert_eq!(Solution::conversion(u64::MAX, u64::MAX), 0);
        assert_eq!(Solution::conversion(0, u64::MAX), 64);
        assert_eq!(Solution::conversion(u64::MAX, 0), 64);
        assert_eq!(Solution::conversion(0b101000, 0b000111), 5);
        assert_eq!(Solution::conversion(0b11101, 0b01111), 2);
        assert_eq!(Solution::conversion(0b101011, 0b100110), 3);
        assert_eq!(Solution::conversion(0b1111000, 0b0001111), 6);
    }
}
