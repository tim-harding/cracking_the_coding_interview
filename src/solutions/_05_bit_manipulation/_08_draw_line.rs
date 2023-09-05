use crate::problems::_05_bit_manipulation::_08_draw_line::DrawLine;

pub struct Solution;

impl DrawLine for Solution {
    fn draw_line(screen: &mut Vec<u8>, width: usize, x1: usize, x2: usize, y: usize) {
        assert!(width % 8 == 0);
        assert!(x1 <= x2);
        assert!(x2 <= width);
        assert!(screen.len() % (width / 8) == 0);
        assert!(y < screen.len() / (width / 8));

        let first_bit = y * width + x1;
        let last_bit = y * width + x2;
        let first_bits_to_fill = u8::MAX >> (first_bit % 8);
        let last_bits_to_fill = !(u8::MAX >> (last_bit % 8));
        let first_byte = first_bit / 8;
        let last_byte = last_bit / 8;
        if first_byte == last_byte {
            screen[first_byte] |= first_bits_to_fill & last_bits_to_fill;
        } else {
            screen[first_byte] |= first_bits_to_fill;
            if let Some(last_byte) = screen.get_mut(last_byte) {
                *last_byte |= last_bits_to_fill;
            }
            for cell in screen
                .iter_mut()
                .skip(first_byte + 1)
                .take(last_byte - first_byte - 1)
            {
                *cell = u8::MAX;
            }
        }
    }
}
