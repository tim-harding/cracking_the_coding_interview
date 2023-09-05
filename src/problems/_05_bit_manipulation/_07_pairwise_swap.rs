pub trait PairwiseSwap {
    /// Swap the even and odd bits of n with as few instructions as possible. By
    /// swapping even and odd bits, we mean to swap bits 0 and 1, 2 and 3, 4 and
    /// 5, and so on.
    fn pairwise_swap(n: u64) -> u64;
}

struct Solution;

impl PairwiseSwap for Solution {
    fn pairwise_swap(n: u64) -> u64 {
        use crate::solutions::_05_bit_manipulation::_07_pairwise_swap as solutions;
        solutions::Solution::pairwise_swap(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swaps_neighboring_bits() {
        assert_eq!(Solution::pairwise_swap(0), 0);
        assert_eq!(Solution::pairwise_swap(u64::MAX), u64::MAX);
        assert_eq!(Solution::pairwise_swap(0b101010), 0b010101);
        assert_eq!(Solution::pairwise_swap(0b010101), 0b101010);
        assert_eq!(Solution::pairwise_swap(0b11110000), 0b11110000);
        assert_eq!(Solution::pairwise_swap(0b111000), 0b110100);
        assert_eq!(Solution::pairwise_swap(0b110001101001), 0b110010010110);
    }
}
