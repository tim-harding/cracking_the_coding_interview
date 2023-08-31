#[derive(Default)]
pub struct AsciiBitset([u64; 4]);

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
