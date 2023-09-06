use crate::problems::_01_arrays_and_strings::_05_one_away::OneAway;
use std::cmp::Ordering;

pub struct Solution;

impl OneAway for Solution {
    fn one_away(a: &str, b: &str) -> bool {
        match a.len().cmp(&b.len()) {
            Ordering::Less => is_deletion(a, b),
            Ordering::Equal => is_edit(a, b),
            Ordering::Greater => is_deletion(b, a),
        }
    }
}

/// a and b should be the same length. Returns true if there are one or zero
/// characters different between the two.
fn is_edit(a: &str, b: &str) -> bool {
    let mut a = a.chars();
    let mut b = b.chars();
    loop {
        match (a.next(), b.next()) {
            (None, Some(_)) | (Some(_), None) => unreachable!(),
            (None, None) => return true,
            (Some(a), Some(b)) => {
                if a != b {
                    break;
                }
            }
        }
    }

    loop {
        match (a.next(), b.next()) {
            (None, Some(_)) | (Some(_), None) => return false,
            (None, None) => return true,
            (Some(a), Some(b)) => {
                if a != b {
                    return false;
                }
            }
        }
    }
}

/// b should be longer than a. Returns true if the strings are the same
/// apart from b containing one extra character.
fn is_deletion(a: &str, b: &str) -> bool {
    let mut a = a.chars().peekable();
    let mut b = b.chars().peekable();
    loop {
        match (a.peek(), b.peek()) {
            (None, None) | (Some(_), None) => unreachable!(),
            (None, Some(_)) => break,
            (Some(a), Some(b)) => {
                if a != b {
                    break;
                }
            }
        }
        a.next();
        b.next();
    }
    b.next();

    loop {
        match (a.next(), b.next()) {
            (None, None) => return true,
            (None, Some(_)) | (Some(_), None) => return false,
            (Some(a), Some(b)) => {
                if a != b {
                    return false;
                }
            }
        }
    }
}
