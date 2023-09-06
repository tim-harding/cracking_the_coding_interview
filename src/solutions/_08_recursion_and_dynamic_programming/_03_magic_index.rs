use crate::problems::_08_recursion_and_dynamic_programming::_03_magic_index::MagicIndex;
use std::cmp::Ordering;

pub struct Solution;

impl MagicIndex for Solution {
    fn magic_index_distinct(array: &[i64]) -> Option<usize> {
        let mut left = 0;
        let mut right = array.len();
        while left < right {
            let mid = left + (right - left) / 2;
            match array[mid].cmp(&(mid as i64)) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => right = mid,
            }
        }
        None
    }

    fn magic_index_indistinct(array: &[i64]) -> Option<usize> {
        array
            .iter()
            .enumerate()
            .find(|(i, &n)| *i as i64 == n)
            .map(|(i, _)| i)
    }
}
