use crate::problems::_05_bit_manipulation::_04_next_number::NextNumber;

pub struct Solution;

impl NextNumber for Solution {
    fn next_lesser(n: u64) -> u64 {
        let ones = n.trailing_ones();
        let zeroes = n.checked_shr(ones).unwrap_or(0).trailing_zeros();
        let total = ones + zeroes;
        if total >= 64 {
            n
        } else {
            let mask = u64::MAX.checked_shl(total + 1).unwrap_or(0);
            let new_leading = !u64::MAX.checked_shl(ones + 1).unwrap_or(0) << (zeroes - 1);
            (n & mask) | new_leading
        }
    }

    fn next_greater(n: u64) -> u64 {
        let zeroes = n.trailing_zeros();
        let ones = n.checked_shr(zeroes).unwrap_or(0).trailing_ones();
        let total = zeroes + ones;
        if total >= 64 {
            n
        } else {
            let mask = u64::MAX << total;
            let shifted_bit = 1 << total;
            let new_carried = !(u64::MAX << (ones - 1));
            (n & mask) | shifted_bit | new_carried
        }
    }
}
