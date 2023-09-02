pub trait Insertion {
    /// Write a method to insert M into N such that M starts at bit j and ends
    /// at bit i. For example,
    /// INPUT:  N = 10000000000, M = 10011, i = 2, j = 6
    /// OUTPUT:     10001001100
    fn insertion(n: u32, m: u32, i: usize, j: usize) -> u32;
}

struct Solution;

impl Insertion for Solution {
    fn insertion(n: u32, m: u32, i: usize, j: usize) -> u32 {
        // Your solution here
        use crate::solutions::_05_bit_manipulation::_01_insertion as solutions;
        solutions::Solution::insertion(n, m, i, j)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_correctly() {
        assert_eq!(
            Solution::insertion(0b10000000000, 0b10011, 2, 6),
            0b10001001100
        );
        assert_eq!(
            Solution::insertion(0b11111111111, 0b0101010, 2, 8),
            0b11010101011
        );
    }
}
