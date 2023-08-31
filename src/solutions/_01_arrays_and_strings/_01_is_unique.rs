use crate::problems::_01_arrays_and_strings::_01_is_unique::IsUnique;
use std::collections::HashSet;

pub struct BasicSolution;

impl IsUnique for BasicSolution {
    fn is_unique(string: &str) -> bool {
        let mut set = HashSet::new();
        for c in string.chars() {
            if !set.insert(c) {
                return false;
            }
        }
        true
    }
}
