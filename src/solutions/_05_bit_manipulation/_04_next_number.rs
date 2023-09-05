use crate::problems::_05_bit_manipulation::_04_next_number::NextNumber;

pub struct Solution;

impl NextNumber for Solution {
    fn next_lesser(n: u64) -> u64 {
        todo!()
    }

    fn next_greater(n: u64) -> u64 {
        let zeroes = n.trailing_zeros();
        let ones = (n >> zeroes).trailing_ones();
        let total = zeroes + ones;
        if total >= 64 {
            n
        } else {
            let mask = u64::MAX << total;
            let shifted_bit = 1 << total;
            let new_carried = !(u64::MAX << (ones - 1));
            println!("{zeroes}, {ones}, {mask:b}, {shifted_bit:b}, {new_carried:b}");
            (n & mask) | shifted_bit | new_carried
        }
    }
}
