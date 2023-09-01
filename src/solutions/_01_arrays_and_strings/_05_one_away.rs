use crate::problems::_01_arrays_and_strings::_05_one_away::OneAway;

pub struct Solution;

impl OneAway for Solution {
    fn one_away(a: &str, b: &str) -> bool {
        if a.len() == b.len() {
            let mut found_edit = false;
            for (a, b) in a.chars().zip(b.chars()) {
                if a != b {
                    if found_edit {
                        return false;
                    }
                    found_edit = true;
                }
            }
            true
        } else if a.len() < b.len() {
            is_deletion(a, b)
        } else {
            is_deletion(b, a)
        }
    }
}

/// b should be longer than a. Returns true if the strings are the same
/// apart from b containing one extra character.
fn is_deletion(a: &str, b: &str) -> bool {
    let mut a = a.chars();
    let mut b = b.chars();
    while let (Some(a), Some(b)) = (a.next(), b.next()) {
        if a != b {
            break;
        }
    }
    b.next();
    while let (Some(a), Some(b)) = (a.next(), b.next()) {
        if a != b {
            return false;
        }
    }
    a.next().is_none() && b.next().is_none()
}
