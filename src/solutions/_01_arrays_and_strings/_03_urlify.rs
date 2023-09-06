use crate::problems::_01_arrays_and_strings::_03_urlify::Urlify;

pub struct Solution;

impl Urlify for Solution {
    fn urlify(string: &mut String, len: usize) {
        let mut bytes = std::mem::take(string).into_bytes();
        let mut dst = bytes.len();
        for i in (0..len).rev() {
            dst -= 1;
            if bytes[i] == b' ' {
                bytes[dst - 2] = b'%';
                bytes[dst - 1] = b'2';
                bytes[dst] = b'0';
                dst -= 2;
            } else {
                bytes[dst] = bytes[i];
            }
        }
        let _ = std::mem::replace(string, String::from_utf8(bytes).unwrap());
    }
}
