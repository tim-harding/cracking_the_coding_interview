pub trait CheckPermutation {
    fn check_permutation(a: &str, b: &str) -> bool;
}

struct Solution;

impl CheckPermutation for Solution {
    fn check_permutation(a: &str, b: &str) -> bool {
        // Replace with your solution
        use crate::solutions::_01_arrays_and_strings::_02_check_permutation::SortingSolution;
        SortingSolution::check_permutation(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_permutations() {
        assert!(Solution::check_permutation("", ""));
        assert!(Solution::check_permutation("triangle", "integral"));
        assert!(Solution::check_permutation("listen", "silent"));
    }
}
