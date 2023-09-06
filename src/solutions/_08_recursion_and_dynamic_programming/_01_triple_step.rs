use crate::problems::_08_recursion_and_dynamic_programming::_01_triple_step::TripleStep;

pub struct Solution;

impl TripleStep for Solution {
    fn triple_step(steps: usize) -> usize {
        match steps {
            0 => 1,
            1 => 1,
            2 => 2,
            3 => 4,
            _ => {
                // This counts the number of ways to get to each stair. Include two
                // dummy stairs to avoid out-of-bounds indexing.
                let mut stairs = vec![0; steps];
                stairs[0] = 1;
                stairs[1] = 2;
                stairs[2] = 4;
                for i in 3..steps {
                    // There is one way to get from the previous stair, two ways (11, 2)
                    // to get from the stair two down, and four ways (111, 21, 12, 3) to
                    // get from the stair three down.
                    stairs[i] = stairs[i - 1] + stairs[i - 2] + stairs[i - 3];
                }
                stairs[steps - 1]
            }
        }
    }
}

pub struct RecursionSolution;

impl TripleStep for RecursionSolution {
    fn triple_step(steps: usize) -> usize {
        match steps {
            0 => 1,
            1 => 1,
            2 => 2,
            _ => {
                Self::triple_step(steps - 1)
                    + Self::triple_step(steps - 2)
                    + Self::triple_step(steps - 3)
            }
        }
    }
}
