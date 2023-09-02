pub trait Urlify {
    /// Write a method to replace all spaces in a string with '%20'. You may
    /// assume that the string has sufficient trailing spaces at the end to hold
    /// the additional characters. The length of the string to escape before
    /// trailing spaces is given by `len`.
    ///
    /// Note that in Rust, because it is not possible to index strings, you will
    /// likely need to convert the string into a Vec first. For the purposes of
    /// the problem, assume that the input will be ASCII-encoded.
    fn urlify(string: &mut String, len: usize);
}

struct Solution;

impl Urlify for Solution {
    fn urlify(string: &mut String, len: usize) {
        // Replace with your solution
        use crate::solutions::_01_arrays_and_strings::_03_urlify as solutions;
        solutions::Solution::urlify(string, len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urlifies() {
        let mut input = "Mr John Smith    ".to_string();
        Solution::urlify(&mut input, 13);
        assert_eq!(input.as_str(), "Mr%20John%20Smith");
    }
}
