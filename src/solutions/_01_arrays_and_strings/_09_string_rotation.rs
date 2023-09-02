use crate::problems::_01_arrays_and_strings::_09_string_rotation::StringRotation;

pub struct Solution;

impl StringRotation for Solution {
    fn string_rotation(s1: &str, s2: &str) -> bool {
        s1.len() == s2.len() && format!("{s1}{s1}").contains(s2)
    }
}
