pub trait FlipBitToWin {
    /// You have an integer and you can flip excactly one bit from a 0 to a 1.
    /// Find the length of the longest sequence of 1s you could create.
    fn flip_bit_to_win(n: u64) -> u8;
}

struct Solution;

impl FlipBitToWin for Solution {
    fn flip_bit_to_win(n: u64) -> u8 {
        // Your solution here
        use crate::solutions::_05_bit_manipulation::_03_flip_bit_to_win as solutions;
        solutions::Solution::flip_bit_to_win(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_correct_sequence_length() {
        assert_eq!(Solution::flip_bit_to_win(0b0), 1);
        assert_eq!(Solution::flip_bit_to_win(0b101), 3);
        assert_eq!(Solution::flip_bit_to_win(0b11011000011), 5);
        assert_eq!(Solution::flip_bit_to_win(0b1001), 2);
        assert_eq!(Solution::flip_bit_to_win(0b11011101111), 8);
        assert_eq!(Solution::flip_bit_to_win(u64::MAX), 64);
        assert_eq!(Solution::flip_bit_to_win(u64::MAX - 0b1), 64);
        assert_eq!(Solution::flip_bit_to_win(u64::MAX - 0b11), 63);
        assert_eq!(Solution::flip_bit_to_win(u64::MAX - 0b1001), 63);
    }
}
