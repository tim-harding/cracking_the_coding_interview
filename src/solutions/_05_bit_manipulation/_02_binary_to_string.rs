use crate::problems::_05_bit_manipulation::_02_binary_to_string::{
    BinaryToString, ConversionError,
};

pub struct Solution;

impl BinaryToString for Solution {
    fn binary_to_string(real: f64) -> Result<String, ConversionError> {
        let mut out = "0.".to_string();
        let mut n = 0.0;
        let mut delta = 1.0;
        for _ in 0..30 {
            delta /= 2.0;
            if n + delta > real {
                out.push('0');
            } else {
                out.push('1');
                n += delta;
            }

            if real == n {
                return Ok(out);
            }
        }
        Err(ConversionError)
    }
}
