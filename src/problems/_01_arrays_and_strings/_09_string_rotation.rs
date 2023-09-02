pub trait StringRotation {
    /// Given two strings, write code to check if s2 is a rotation of s1 using
    /// only one call to String::contains. An example of a rotation would be
    /// waterbottle and erbottlewat.
    fn string_rotation(s1: &str, s2: &str) -> bool;
}

struct Solution;

impl StringRotation for Solution {
    fn string_rotation(s1: &str, s2: &str) -> bool {
        // Replace with your solution
        use crate::solutions::_01_arrays_and_strings::_09_string_rotation as solutions;
        solutions::Solution::string_rotation(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirms_string_rotations() {
        assert!(Solution::string_rotation("", ""));
        assert!(Solution::string_rotation("a", "a"));
        assert!(Solution::string_rotation("aaa", "aaa"));
        assert!(Solution::string_rotation("abcde", "cdeab"));
        assert!(Solution::string_rotation("waterbottle", "erbottlewat"));
        assert!(Solution::string_rotation("erbottlewat", "waterbottle"));
    }

    #[test]
    fn rejects_non_rotations() {
        assert!(!Solution::string_rotation("abcde", "abced"));
        assert!(!Solution::string_rotation("apple", "app"));
    }
}
