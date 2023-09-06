use crate::problems::_08_recursion_and_dynamic_programming::_03_magic_index::MagicIndex;
use std::cmp::Ordering;

pub struct Solution;

impl MagicIndex for Solution {
    fn magic_index_distinct(array: &[i64]) -> Option<usize> {
        Self::magic_index_inner(array, 0, array.len() - 1)
    }

    fn magic_index_indistinct(array: &[i64]) -> Option<usize> {
        array
            .iter()
            .enumerate()
            .find(|(i, &n)| *i as i64 == n)
            .map(|(i, _)| i)
    }
}

impl Solution {
    fn magic_index_inner(array: &[i64], left: usize, right: usize) -> Option<usize> {
        if right <= left {
            return None;
        }

        match (
            array[left].cmp(&(left as i64)),
            array[right].cmp(&(right as i64)),
        ) {
            // Can't find the magic index if the subarray does not contain it
            (Ordering::Less, Ordering::Less) | (Ordering::Greater, Ordering::Greater) => None,
            (Ordering::Equal, _) => Some(left),
            (_, Ordering::Equal) => Some(right),
            (Ordering::Less, Ordering::Greater) | (Ordering::Greater, Ordering::Less) => {
                let mid = left + (right - left) / 2;
                Self::magic_index_inner(array, left, mid)
                    .or_else(|| Self::magic_index_inner(array, mid + 1, right))
            }
        }
    }
}
