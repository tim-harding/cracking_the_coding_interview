use crate::problems::_05_bit_manipulation::_06_conversion::Conversion;

pub struct Solution;

impl Conversion for Solution {
    fn conversion(a: u64, b: u64) -> u8 {
        (a ^ b).count_ones() as u8
    }
}
