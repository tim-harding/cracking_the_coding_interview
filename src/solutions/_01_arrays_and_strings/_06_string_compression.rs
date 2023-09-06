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
        let mut count = 1usize;
        let mut out = String::new();
        for c in chars {
            if c != previous {
                out.push(previous);
                append_count(&mut out, count);
                previous = c;
                count = 1;
            } else {
                count += 1;
            }
        }
        out.push(previous);
        append_count(&mut out, count);
        if out.len() > string.len() {
            Cow::Borrowed(string)
        } else {
            Cow::Owned(out)
        }
    }
}

fn append_count(s: &mut String, mut count: usize) {
    // SAFETY: We are just appending ASCII characters, so this will yield valid
    // UTF-8. We do this to avoid allocating with the format!() macro.
    let v = unsafe { s.as_mut_vec() };
    let mut count_chars = 0;
    while count > 0 {
        v.push((count % 10) as u8 + b'0');
        count /= 10;
        count_chars += 1;
    }
    for i in 0..count_chars / 2 {
        let len = v.len();
        v.swap(len - count_chars + i, len - i - 1)
    }
}
