use crate::problems::_08_recursion_and_dynamic_programming::_02_robot_on_a_grid::{
    Floor, RobotOnAGrid, Step,
};

pub struct Solution;

impl RobotOnAGrid for Solution {
    fn robot_on_a_grid(floor: &Floor) -> Option<Vec<Step>> {
        let index = |x, y| y * floor.width() + x;
        let mut access = vec![None; floor.width() * floor.height()];
        for y in 1..floor.height() {
            if floor.is_off_limits(0, y) {
                break;
            }
            access[y * floor.width()] = Some(Step::Down);
        }

        for x in 1..floor.width() {
            if floor.is_off_limits(x, 0) {
                break;
            }
            access[x] = Some(Step::Right);
        }

        for y in 1..floor.height() {
            for x in 1..floor.width() {
                let step = if floor.is_off_limits(x, y) {
                    continue;
                } else if access[index(x, y - 1)].is_some() {
                    Step::Down
                } else if access[index(x - 1, y)].is_some() {
                    Step::Right
                } else {
                    continue;
                };
                access[index(x, y)] = Some(step);
            }
        }

        access[floor.height() * floor.width() - 1]?;

        let mut path = vec![Step::Right; floor.width() + floor.height() - 2];

        let mut i = path.len();
        let mut x = floor.width() - 1;
        let mut y = floor.height() - 1;
        while i > 0 {
            i -= 1;
            let step = access[index(x, y)].unwrap();
            match step {
                Step::Right => x -= 1,
                Step::Down => y -= 1,
            }
            path[i] = step;
        }

        Some(path)
    }
}
