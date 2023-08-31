use crate::problems::_01_arrays_and_strings::_01_is_unique::IsUnique;
use std::collections::HashSet;

pub struct HashsetSolution;

impl IsUnique for HashsetSolution {
    fn is_unique(string: &str) -> bool {
        let mut set = HashSet::new();
        for c in string.chars() {
            if !set.insert(c) {
                return false;
            }
        }
        true
    }
}

pub struct AsciiSolution;

impl IsUnique for AsciiSolution {
    fn is_unique(string: &str) -> bool {
        let mut bitset = AsciiBitset::new();
        for c in string.chars() {
            if !c.is_ascii() {
                panic!("Expected an ASCII character");
            }
            if bitset.get(c) {
                return false;
            }
            bitset.set(c);
        }
        true
    }
}

#[derive(Default)]
struct AsciiBitset([u64; 4]);

impl AsciiBitset {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, c: char) -> bool {
        let i = c as usize;
        ((self.0[i / 64] >> (i % 64)) & 1) == 1
    }

    pub fn set(&mut self, c: char) {
        let i = c as usize;
        self.0[i / 64] |= 1 << (i % 64);
    }
}
