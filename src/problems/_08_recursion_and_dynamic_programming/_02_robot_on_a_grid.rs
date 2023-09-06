use std::fmt::{self, Formatter};

pub trait RobotOnAGrid {
    /// Imagine a robot sitting on the upper left corner of a floor. The robot
    /// can move in two directions, right and down, but certain cells are off
    /// limits such that the robot cannot step on them. Find a path for the
    /// robot from top left to bottom right, if possible.
    fn robot_on_a_grid(floor: &Floor) -> Option<Vec<Step>>;
}

struct Solution;

impl RobotOnAGrid for Solution {
    fn robot_on_a_grid(floor: &Floor) -> Option<Vec<Step>> {
        // Your solution here
        use crate::solutions::_08_recursion_and_dynamic_programming::_02_robot_on_a_grid as solutions;
        solutions::Solution::robot_on_a_grid(floor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Step {
    Right,
    Down,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Floor {
    cells: Vec<u64>,
    width: usize,
    height: usize,
}

impl Floor {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![0; (width * height + 7) / 8],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_off_limits(&self, x: usize, y: usize) -> bool {
        assert!(x < self.width);
        assert!(y < self.height);
        let bit = y * self.width + x;
        (self.cells[bit / 8] >> (bit % 8)) & 1 == 1
    }

    pub fn set_off_limits(&mut self, x: usize, y: usize) {
        assert!(x < self.width);
        assert!(y < self.height);
        let bit = y * self.width + x;
        self.cells[bit / 8] |= 1 << (bit % 8);
    }

    pub fn is_valid_path(&self, steps: &[Step]) -> bool {
        let mut x = 0;
        let mut y = 0;
        if self.is_off_limits(0, 0) {
            return false;
        }

        for step in steps {
            match step {
                Step::Right => {
                    x += 1;
                    if x >= self.width {
                        return false;
                    }
                }

                Step::Down => {
                    y += 1;
                    if y >= self.height {
                        return false;
                    }
                }
            }

            if self.is_off_limits(x, y) {
                return false;
            }
        }

        x == self.width - 1 && y == self.height - 1
    }
}

impl fmt::Debug for Floor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.is_off_limits(x, y) { 1 } else { 0 })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_trivial_path() {
        let floor = Floor::new(4, 4);
        let path = Solution::robot_on_a_grid(&floor).unwrap();
        assert!(floor.is_valid_path(&path));
    }

    #[test]
    fn fails_to_find_impossible_path() {
        #[rustfmt::skip]
        let floor = floor_from_bits(&[
            0b00000000,
            0b00000011,
            0b00000100,
            0b00001000,
            0b00010000,
            0b00100000,
            0b11000000,
            0b00000000,
        ]);
        assert_eq!(Solution::robot_on_a_grid(&floor), None);
    }

    #[test]
    fn finds_only_paths() {
        #[rustfmt::skip]
        let floor = floor_from_bits(&[
            0b01000000,
            0b00000000,
            0b00000010,
        ]);
        assert_eq!(
            Solution::robot_on_a_grid(&floor),
            Some(vec![
                Step::Down,
                Step::Right,
                Step::Right,
                Step::Right,
                Step::Right,
                Step::Right,
                Step::Right,
                Step::Right,
                Step::Down,
            ])
        );

        #[rustfmt::skip]
        let floor = floor_from_bits(&[
            0b01000010,
            0b01000010,
            0b00000010,
            0b00001010,
            0b00001000,
        ]);
        assert_eq!(
            Solution::robot_on_a_grid(&floor),
            Some(vec![
                Step::Down,
                Step::Down,
                Step::Right,
                Step::Right,
                Step::Right,
                Step::Right,
                Step::Right,
                Step::Down,
                Step::Down,
                Step::Right,
                Step::Right,
            ])
        );

        #[rustfmt::skip]
        let floor = floor_from_bits(&[
            0b00000010,
            0b11111010,
            0b00001010,
            0b00001000,
            0b00001110,
        ]);
        assert_eq!(
            Solution::robot_on_a_grid(&floor),
            Some(vec![
                Step::Right,
                Step::Right,
                Step::Right,
                Step::Right,
                Step::Right,
                Step::Down,
                Step::Down,
                Step::Down,
                Step::Right,
                Step::Right,
                Step::Down,
            ])
        );
    }

    fn floor_from_bits(rows: &[u8]) -> Floor {
        let mut floor = Floor::new(8, rows.len());
        for (y, row) in rows.iter().enumerate() {
            for x in 0..8 {
                let is_off_limits = (row >> (7 - x)) & 1 == 1;
                if is_off_limits {
                    floor.set_off_limits(x, y)
                }
            }
        }
        floor
    }
}
