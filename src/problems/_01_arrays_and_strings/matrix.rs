use std::fmt::{self, Debug, Formatter};

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix {
    /// Matrix elements
    pub cells: Vec<u8>,
    /// Number of rows
    pub m: usize,
    /// Number of columns
    pub n: usize,
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
