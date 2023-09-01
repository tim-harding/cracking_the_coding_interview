use std::borrow::Cow;

pub trait StringCompression {
    /// Implement a method to perform basic string compression using the counts
    /// of repeated characters. For example, the string aabcccccaaa would become
    /// a2b1c5a3. If the compressed string would not become smaler than the
    /// original string, your method should return the original string. You can
    /// assume the string has only uppercase and lowercase letters.
    fn string_compression(string: &str) -> Cow<str>;
}

struct Solution;

impl StringCompression for Solution {
    fn string_compression(string: &str) -> Cow<str> {
        // Replace with your solution
        use crate::solutions::_01_arrays_and_strings::_06_string_compression as solutions;
        solutions::Solution::string_compression(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compresses() {
        assert_eq!(Solution::string_compression("aabcccccaaa"), "a2b1c5a3");
        assert_eq!(Solution::string_compression("aaaaaaaaaaaa"), "a12");
        assert_eq!(
            Solution::string_compression(format!("{:a<1$}", "", 105).as_str()),
            "a105"
        );
    }

    #[test]
    fn returns_original_string() {
        assert_eq!(Solution::string_compression("abca"), "abca");
        assert_eq!(Solution::string_compression("aabbcca"), "aabbcca");
    }
}
