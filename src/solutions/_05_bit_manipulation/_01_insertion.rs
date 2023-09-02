use crate::problems::_05_bit_manipulation::_01_insertion::Insertion;

pub struct Solution;

impl Insertion for Solution {
    fn insertion(n: u32, m: u32, i: usize, j: usize) -> u32 {
        let m_mask = u32::MAX << (j - i + 1);
        let m = (m & !m_mask) << i;
        let n_mask = m_mask << i | !(u32::MAX << i);
        let n = n & n_mask;
        m | n
    }
}
