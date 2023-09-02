use crate::problems::_05_bit_manipulation::_03_flip_bit_to_win::FlipBitToWin;

pub struct Solution;

impl FlipBitToWin for Solution {
    fn flip_bit_to_win(n: u64) -> u8 {
        let mut best = 0;
        let mut current = 0;
        let mut previous = 0;
        for i in 0..64 {
            if (n >> i) & 1 == 1 {
                current += 1;
            } else {
                best = best.max(current + previous);
                previous = current;
                current = 0;
            }
        }
        best = best.max(current + previous);
        best + 1
    }
}
