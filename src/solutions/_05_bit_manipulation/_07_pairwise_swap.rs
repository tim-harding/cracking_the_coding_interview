use crate::problems::_05_bit_manipulation::_07_pairwise_swap::PairwiseSwap;

pub struct Solution;

const EVEN_MASK: u64 = 0b1010101010101010101010101010101010101010101010101010101010101010;
const ODD_MASK: u64 = !EVEN_MASK;

impl PairwiseSwap for Solution {
    fn pairwise_swap(n: u64) -> u64 {
        ((n & EVEN_MASK) >> 1) | ((n & ODD_MASK) << 1)
    }
}
