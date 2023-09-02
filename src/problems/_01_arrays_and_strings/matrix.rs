use std::{
    fmt::{self, Debug, Formatter},
    ops::{Deref, Index, IndexMut},
};

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix {
    /// Matrix elements
    pub cells: Vec<u8>,
    /// Number of rows
    pub m: usize,
    /// Number of columns
    pub n: usize,
}

impl Matrix {
    pub fn new(n: usize, m: usize, cells: Vec<u8>) -> Self {
        Self { n, m, cells }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self[y * self.n + x]
    }

    pub fn set(&mut self, x: usize, y: usize, cell: u8) {
        let n = self.n;
        self[y * n + x] = cell;
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cells: Vec<_> = self.cells.iter().map(|cell| format!("{cell}")).collect();
        let column_widths: Vec<_> = (0..self.n)
            .map(|i| {
                cells
                    .iter()
                    .skip(i)
                    .step_by(self.n)
                    .map(|cell| cell.len())
                    .max()
                    .unwrap()
            })
            .collect();

        for chunk in cells.chunks(self.n) {
            for (column, cell) in chunk.iter().enumerate() {
                write!(f, "{:>1$}", cell, column_widths[column] + 1)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Deref for Matrix {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.cells.as_slice()
    }
}

impl Index<usize> for Matrix {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}
