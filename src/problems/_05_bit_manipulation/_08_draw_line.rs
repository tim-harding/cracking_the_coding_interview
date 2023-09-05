pub trait DrawLine {
    /// A monochrome screen is stored as a single array of bytes, allowing eight
    /// consecutive pixels to be stored in one byte. The screen width is
    /// divisible by 8. Implement a function that draws a horizontal line from
    /// (x1, y) to (x2, y) exclusive.
    fn draw_line(screen: &mut Vec<u8>, width: usize, x1: usize, x2: usize, y: usize);
}

struct Solution;

impl DrawLine for Solution {
    fn draw_line(screen: &mut Vec<u8>, width: usize, x1: usize, x2: usize, y: usize) {
        use crate::solutions::_05_bit_manipulation::_08_draw_line as solutions;
        solutions::Solution::draw_line(screen, width, x1, x2, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draws_lines() {
        let mut screen = vec![0];
        Solution::draw_line(&mut screen, 8, 4, 4, 0);
        assert_eq!(screen, vec![0]);

        screen = vec![0];
        Solution::draw_line(&mut screen, 8, 0, 8, 0);
        assert_eq!(screen, vec![u8::MAX]);

        screen = vec![0b10101010, 0b10101010];
        Solution::draw_line(&mut screen, 16, 5, 10, 0);
        assert_eq!(screen, vec![0b10101111, 0b11101010]);

        screen = vec![0, 0, 0, 0];
        Solution::draw_line(&mut screen, 8, 4, 8, 1);
        assert_eq!(screen, vec![0, 0b00001111, 0, 0]);

        screen = vec![0, 0, 0, 0, 0, 0];
        Solution::draw_line(&mut screen, 16, 4, 12, 1);
        assert_eq!(screen, vec![0, 0, 0b00001111, 0b11110000, 0, 0]);

        screen = vec![0, 0, 0, 0, 0];
        Solution::draw_line(&mut screen, 40, 3, 37, 0);
        assert_eq!(screen, vec![0b00011111, 255, 255, 255, 0b11111000]);
    }
}
