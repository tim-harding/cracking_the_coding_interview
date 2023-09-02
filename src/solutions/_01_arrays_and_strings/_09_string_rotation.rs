use crate::problems::_01_arrays_and_strings::_09_string_rotation::StringRotation;

pub struct Solution;

impl StringRotation for Solution {
    fn string_rotation(s1: &str, s2: &str) -> bool {
        // If s1 and s2 are rotations of one another, then there exists some
        // pair of substrings x and y such that s1 is the concatenation xy and
        // s2 is the concatenation yx. We use the fact that yx is a substring of
        // xyxy to find the solution.
        s1.len() == s2.len() && format!("{s1}{s1}").contains(s2)
    }
}
