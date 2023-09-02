use std::fmt::{self, Display, Formatter};

pub trait BinaryToString {
    /// Given a real number between 0 and 1, return the binary representation.
    /// For example, 0.625 is 0.101 in binary because 1/2 + 0/4 + 1/8 = 0.625.
    /// If the number cannot be represented accurately in binary with at most 32
    /// characters, return an error.
    fn binary_to_string(real: f64) -> Result<String, ConversionError>;
}

struct Solution;

impl BinaryToString for Solution {
    fn binary_to_string(real: f64) -> Result<String, ConversionError> {
        // Your solution here
        use crate::solutions::_05_bit_manipulation::_02_binary_to_string as solutions;
        solutions::Solution::binary_to_string(real)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionError;

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to convert the real number in at most 32 characters"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_reals_to_binary() {
        assert_eq!(Solution::binary_to_string(0.0), Ok("0.0".to_string()));
        assert_eq!(Solution::binary_to_string(0.5), Ok("0.1".to_string()));
        assert_eq!(Solution::binary_to_string(0.25), Ok("0.01".to_string()));
        assert_eq!(Solution::binary_to_string(0.75), Ok("0.11".to_string()));
        assert_eq!(Solution::binary_to_string(0.625), Ok("0.101".to_string()));
        assert_eq!(Solution::binary_to_string(0.125), Ok("0.001".to_string()));
    }

    #[test]
    fn fails_for_long_binary_representation() {
        assert_eq!(
            Solution::binary_to_string(0.72513749782),
            Err(ConversionError)
        )
    }
}
