use crate::problems::_08_recursion_and_dynamic_programming::_01_triple_step::TripleStep;

pub struct Solution;

impl TripleStep for Solution {
    fn triple_step(steps: usize) -> usize {
        let mut stairs = vec![0; steps.max(3)];
        stairs[0] = 1;
        stairs[1] = 2;
        stairs[2] = 4;
        for i in 3..steps {
            stairs[i] = stairs[i - 1] + stairs[i - 2] + stairs[i - 3];
        }
        stairs[steps - 1]
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

pub struct MemoizedSolution;

impl TripleStep for MemoizedSolution {
    fn triple_step(steps: usize) -> usize {
        Self::triple_step_inner(steps, &mut vec![0; steps + 1])
    }
}

impl MemoizedSolution {
    fn triple_step_inner(steps: usize, memo: &mut Vec<usize>) -> usize {
        match steps {
            0 => 1,
            1 => 1,
            2 => 2,
            _ => {
                if memo[steps] == 0 {
                    memo[steps] = Self::triple_step_inner(steps - 1, memo)
                        + Self::triple_step_inner(steps - 2, memo)
                        + Self::triple_step_inner(steps - 3, memo);
                }
                memo[steps]
            }
        }
    }
}
