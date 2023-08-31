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

pub struct AsciiSolution;

impl CheckPermutation for AsciiSolution {
    fn check_permutation(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut counts = [0u16; 128];
        for c in a.chars() {
            if !c.is_ascii() {
                panic!("Expected an ASCII character");
            }
            let i = c as usize;
            counts[i] += 1;
        }

        for c in b.chars() {
            if !c.is_ascii() {
                panic!("Expected an ASCII character");
            }
            let i = c as usize;
            if counts[i] == 0 {
                return false;
            }
            counts[i] -= 1;
        }

        true
    }
}
