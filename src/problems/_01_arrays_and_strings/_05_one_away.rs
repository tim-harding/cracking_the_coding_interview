pub trait OneAway {
    /// There are three types of edits that can be performed on strings: insert
    /// a character, remove a character, or replace a character. Given two
    /// strings, write a function to check if they are one or zero edits apart.
    fn one_away(a: &str, b: &str) -> bool;
}

struct Solution;

impl OneAway for Solution {
    fn one_away(a: &str, b: &str) -> bool {
        // Replace with your solution
        use crate::solutions::_01_arrays_and_strings::_05_one_away as solutions;
        solutions::Solution::one_away(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_zero_away() {
        assert!(Solution::one_away("pale", "pale"));
        assert!(Solution::one_away("", ""));
    }

    #[test]
    fn accepts_one_away() {
        assert!(Solution::one_away("pale", "ple"));
        assert!(Solution::one_away("pales", "pale"));
        assert!(Solution::one_away("pale", "bale"));
    }

    #[test]
    fn rejects_more_away() {
        assert!(!Solution::one_away("pale", "bae"));
    }
}
