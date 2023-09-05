use crate::solutions::_05_bit_manipulation::_04_next_number as solutions;

pub trait NextNumber {
    /// Given a positive integer, return the next smallest and the next largest
    /// number that have the same number of 1 bits in their binary
    /// representation. If no such number exists, return the original number.
    fn next_lesser(n: u64) -> u64;
    fn next_greater(n: u64) -> u64;
}

struct Solution;

impl NextNumber for Solution {
    fn next_lesser(n: u64) -> u64 {
        // Your solution here
        solutions::Solution::next_lesser(n)
    }

    fn next_greater(n: u64) -> u64 {
        // Your solution here
        solutions::Solution::next_greater(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_original_for_no_greater() {
        assert_eq!(Solution::next_greater(0b0), 0b0);
        assert_eq!(Solution::next_greater(u64::MAX), u64::MAX);
        assert_eq!(Solution::next_greater(u64::MAX - 1), u64::MAX - 1);
        assert_eq!(Solution::next_greater(u64::MAX - 3), u64::MAX - 3);
    }

    #[test]
    fn returns_original_for_no_lesser() {
        assert_eq!(Solution::next_lesser(0b0), 0b0);
        assert_eq!(Solution::next_lesser(0b111), 0b111);
        assert_eq!(Solution::next_lesser(u64::MAX), u64::MAX);
    }

    #[test]
    fn finds_next_greater() {
        assert_eq!(Solution::next_greater(0b10), 0b100);
        assert_eq!(Solution::next_greater(0b110), 0b1001);
        assert_eq!(Solution::next_greater(0b1110), 0b10011);
        assert_eq!(Solution::next_greater(0b1010), 0b1100);
        assert_eq!(Solution::next_greater(0b1100), 0b10001);
        assert_eq!(Solution::next_greater(0b10011), 0b10101);
        assert_eq!(Solution::next_greater(0b10101), 0b10110);
        assert_eq!(Solution::next_greater(0b100111), 0b101011);
        assert_eq!(Solution::next_greater(0b100111000), 0b101000011);
        assert_eq!(Solution::next_greater(0b10010011), 0b10010101);
        assert_eq!(Solution::next_greater(0b100110011), 0b100110101);
        assert_eq!(Solution::next_greater(0b1000011), 0b1000101);
    }

    #[test]
    fn finds_next_lesser() {
        assert_eq!(Solution::next_lesser(0b10), 0b1);
        assert_eq!(Solution::next_lesser(0b110), 0b101);
        assert_eq!(Solution::next_lesser(0b1110), 0b1101);
        assert_eq!(Solution::next_lesser(0b1010), 0b1001);
        assert_eq!(Solution::next_lesser(0b1100), 0b1010);
        assert_eq!(Solution::next_lesser(0b10011), 0b1110);
        assert_eq!(Solution::next_lesser(0b10101), 0b10011);
        assert_eq!(Solution::next_lesser(0b100111), 0b11110);
        assert_eq!(Solution::next_lesser(0b100111000), 0b100110100);
        assert_eq!(Solution::next_lesser(0b10010011), 0b10001110);
        assert_eq!(Solution::next_lesser(0b100110011), 0b100101110);
        assert_eq!(Solution::next_lesser(0b1000011), 0b111000);
    }
}
