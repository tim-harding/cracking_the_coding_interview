use crate::problems::_01_arrays_and_strings::_02_check_permutation::CheckPermutation;

pub struct SortingSolution;

impl CheckPermutation for SortingSolution {
    fn check_permutation(a: &str, b: &str) -> bool {
        let mut a: Vec<_> = a.chars().collect();
        let mut b: Vec<_> = b.chars().collect();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    }
}
