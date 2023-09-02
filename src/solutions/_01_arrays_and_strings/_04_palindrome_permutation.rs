use crate::problems::_01_arrays_and_strings::_04_palindrome_permutation::PalindromePermutation;
use std::collections::{hash_map::Entry, HashMap};

pub struct HashMapSolution;

impl PalindromePermutation for HashMapSolution {
    fn palindrome_permutation(string: &str) -> bool {
        let mut letter_counts = HashMap::new();
        let mut char_count = 0;
        for c in string.chars() {
            if c.is_ascii_whitespace() {
                continue;
            }
            if !c.is_ascii_alphabetic() {
                return false;
            }
            char_count += 1;

            match letter_counts.entry(c.to_lowercase().next().unwrap()) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                }
                Entry::Vacant(entry) => {
                    entry.insert(1usize);
                }
            }
        }

        // If the length of the string is odd, we may have one character whose
        // count is odd as well. If we find an odd count, no other characters
        // may have an odd count.
        let mut allow_odd_count = char_count % 2 == 1;
        for count in letter_counts.values() {
            let odd = count % 2 == 1;
            if odd && !allow_odd_count {
                return false;
            }
            if odd {
                allow_odd_count = false;
            }
        }
        true
    }
}

pub struct BitsetSolution;

impl PalindromePermutation for BitsetSolution {
    fn palindrome_permutation(string: &str) -> bool {
        // Letters is a bitset of whether each letter has an odd count.
        let mut letters = 0u32;
        let mut char_count = 0;
        for c in string.chars() {
            if c.is_ascii_whitespace() {
                continue;
            }
            if !c.is_ascii_alphabetic() {
                return false;
            }
            char_count += 1;
            let c = c.to_lowercase().next().unwrap();
            letters ^= 1 << (c as usize - 'a' as usize);
        }
        let odd_letters = letters.count_ones();
        // If the string length is odd, we may have exactly one odd character
        // count. Otherwise, all letters must be of even count.
        (char_count % 2 == 0 && odd_letters == 0) || (char_count % 2 == 1 && odd_letters == 1)
    }
}
