use crate::problems::_01_arrays_and_strings::_06_string_compression::StringCompression;
use std::borrow::Cow;

pub struct Solution;

impl StringCompression for Solution {
    fn string_compression(string: &str) -> Cow<str> {
        let mut chars = string.chars();
        let mut previous = match chars.next() {
            Some(c) => c,
            None => return Cow::Borrowed(string),
        };
        loop {
            match chars.next() {
                Some(c) => {
                    if c == previous {
                        break;
                    }
                    previous = c;
                }
                None => return Cow::Borrowed(string),
            }
        }

        let mut chars = string.chars();
        let mut previous = chars.next().unwrap();
        let mut count = 1u32;
        let mut out = String::new();
        for c in chars {
            if c != previous {
                out.extend(format!("{previous}{count}").chars());
                previous = c;
                count = 1;
            } else {
                count += 1;
            }
        }
        out.extend(format!("{previous}{count}").chars());
        Cow::Owned(out)
    }
}
