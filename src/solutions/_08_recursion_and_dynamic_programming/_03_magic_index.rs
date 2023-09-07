use crate::problems::_08_recursion_and_dynamic_programming::_03_magic_index::MagicIndex;
use std::cmp::Ordering;

pub struct Solution;

impl MagicIndex for Solution {
    fn magic_index_distinct(array: &[i64]) -> Option<usize> {
        // Binary search for the magic index
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
        Self::magic_index_indistinct_inner(array, 0, array.len() - 1)
    }
}

impl Solution {
    fn magic_index_indistinct_inner(array: &[i64], left: usize, right: usize) -> Option<usize> {
        // If the elements are not unique, we can't use binary search as before.
        // Either side could contain the magic index so we search both sides.
        // However, we can narrow the search by noting that if, for example,
        // array[5] = 3, then array[4] cannot be the magic index. Similarly, if
        // array[2] = 5, then neither array[3] nor array[4] can be the magic
        // index.
        if right < left {
            return None;
        }

        let mid = left + (right - left) / 2;
        let mid_value = array[mid];
        if mid_value == mid as i64 {
            return Some(mid);
        }

        if let Ok(upper_bound) = (mid as i64 - 1).min(mid_value).try_into() {
            if let Some(magic) = Self::magic_index_indistinct_inner(array, left, upper_bound) {
                return Some(magic);
            }
        }

        if let Ok(mid_value) = mid_value.try_into() {
            let lower_bound = (mid + 1).max(mid_value);
            if let Some(magic) = Self::magic_index_indistinct_inner(array, lower_bound, right) {
                return Some(magic);
            }
        }

        None
    }
}
