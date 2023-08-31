pub trait IsUnique {
    /// Implement an algorithm to determine if a string has all unique
    /// characters. What if you cannot use additional data structures?
    fn is_unique(string: &str) -> bool;
}

struct Solution;

impl IsUnique for Solution {
    fn is_unique(string: &str) -> bool {
        // Replace with your solution
        use crate::solutions::_01_arrays_and_strings::_01_is_unique::HashsetSolution;
        HashsetSolution::is_unique(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_unique() {
        assert!(Solution::is_unique(""));
        assert!(Solution::is_unique("abcdeABCDE"));
    }

    #[test]
    fn rejects_duplicates() {
        assert!(!Solution::is_unique("banana"));
        assert!(!Solution::is_unique("tepid and unwavering"));
    }
}
