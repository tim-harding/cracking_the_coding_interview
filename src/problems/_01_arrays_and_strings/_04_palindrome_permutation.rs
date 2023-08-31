pub trait PalindromePermutation {
    /// Given a string, write a function to check if it is a permutation of a
    /// palindrome. A palindrome is a word or phrase that is the same forwards
    /// and backwards. A permutation is a rearrangement of letters. The
    /// palindrome does not need to be limited to just dictionary words.
    fn palindrome_permutation(string: &str) -> bool;
}

struct Solution;

impl PalindromePermutation for Solution {
    fn palindrome_permutation(string: &str) -> bool {
        // Replace with your solution
        use crate::solutions::_01_arrays_and_strings::_04_palindrome_permutation as solutions;
        solutions::Solution::palindrome_permutation(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirms_palidrome_permutations() {
        assert!(Solution::palindrome_permutation(""));
        assert!(Solution::palindrome_permutation("abcba"));
        assert!(Solution::palindrome_permutation("abccba"));
        // taco cat
        assert!(Solution::palindrome_permutation("Tact Coa"));
        // Mr Owl ate my metal worm
        assert!(Solution::palindrome_permutation("worm metal ate my owl mr"));
        // Never odd or even
        assert!(Solution::palindrome_permutation("rddrneooevveen"));
    }

    #[test]
    fn rejects_non_palindromes() {
        assert!(!Solution::palindrome_permutation("Charmander"));
        assert!(!Solution::palindrome_permutation("Telegraph"));
    }
}
