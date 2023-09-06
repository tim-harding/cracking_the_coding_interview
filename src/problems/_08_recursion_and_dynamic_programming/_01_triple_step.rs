pub trait TripleStep {
    /// A child is running up a staircase with n steps and can hop either one,
    /// two, or three steps at a time. Count the number of ways the child can
    /// run up the stairs.
    fn triple_step(steps: usize) -> usize;
}

struct Solution;

impl TripleStep for Solution {
    fn triple_step(steps: usize) -> usize {
        // Your solution here
        use crate::solutions::_08_recursion_and_dynamic_programming::_01_triple_step as solutions;
        solutions::Solution::triple_step(steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_number_of_climbs() {
        // 1
        assert_eq!(Solution::triple_step(1), 1);

        // 11
        // 2
        assert_eq!(Solution::triple_step(2), 2);

        // 111
        // 21
        // 12
        // 3
        assert_eq!(Solution::triple_step(3), 4);

        // 1111
        // 211
        // 121
        // 112
        // 22
        // 31
        // 13
        assert_eq!(Solution::triple_step(4), 7);

        // 11111
        // 2111
        // 1211
        // 1121
        // 1112
        // 122
        // 212
        // 221
        // 311
        // 131
        // 113
        // 32
        // 23
        assert_eq!(Solution::triple_step(5), 13);

        // 111111
        // 21111
        // 12111
        // 11211
        // 11121
        // 11112
        // 2211
        // 2121
        // 2112
        // 1221
        // 1212
        // 1122
        // 3111
        // 1311
        // 1131
        // 1113
        // 222
        // 123
        // 213
        // 132
        // 321
        // 231
        // 312
        // 33
        assert_eq!(Solution::triple_step(6), 24);
    }
}
