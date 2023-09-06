use crate::solutions::_08_recursion_and_dynamic_programming::_03_magic_index as solutions;

pub trait MagicIndex {
    /// Given a sorted array of distinct integers, find an index such that
    /// array[i] = i.
    fn magic_index_distinct(array: &[i64]) -> Option<usize>;

    /// Given a sorted array of integers, find an index such that
    /// array[i] = i.
    fn magic_index_indistinct(array: &[i64]) -> Option<usize>;
}

struct Solution;

impl MagicIndex for Solution {
    fn magic_index_distinct(array: &[i64]) -> Option<usize> {
        // Your solution here
        solutions::Solution::magic_index_distinct(array)
    }

    fn magic_index_indistinct(array: &[i64]) -> Option<usize> {
        // Your solution here
        solutions::Solution::magic_index_indistinct(array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_magic_index() {
        assert_eq!(Solution::magic_index_distinct(&[1, 2, 3, 4, 5]), None);
        assert_eq!(Solution::magic_index_distinct(&[-1, 0, 1, 2, 3]), None);
        assert_eq!(Solution::magic_index_distinct(&[-4, -3, -2, -1, 0]), None);
    }

    #[test]
    fn finds_magic_index_distinct() {
        assert_eq!(
            Solution::magic_index_distinct(&[-6, -4, -2, 0, 2, 4, 6, 8]),
            Some(6)
        );

        // Testing magic index in each position for a even-length array
        assert_eq!(Solution::magic_index_distinct(&[0, 2, 3, 4, 5, 6]), Some(0));
        assert_eq!(
            Solution::magic_index_distinct(&[-1, 1, 3, 4, 5, 6]),
            Some(1)
        );
        assert_eq!(
            Solution::magic_index_distinct(&[-1, 0, 2, 4, 5, 6]),
            Some(2)
        );
        assert_eq!(
            Solution::magic_index_distinct(&[-1, 0, 1, 3, 5, 6]),
            Some(3)
        );
        assert_eq!(
            Solution::magic_index_distinct(&[-1, 0, 1, 2, 4, 6]),
            Some(4)
        );
        assert_eq!(
            Solution::magic_index_distinct(&[-1, 0, 1, 2, 3, 5]),
            Some(5)
        );

        // Testing magic index in each position for a odd-length array
        assert_eq!(
            Solution::magic_index_distinct(&[0, 2, 3, 4, 5, 6, 7]),
            Some(0)
        );
        assert_eq!(
            Solution::magic_index_distinct(&[-1, 1, 3, 4, 5, 6, 7]),
            Some(1)
        );
        assert_eq!(
            Solution::magic_index_distinct(&[-1, 0, 2, 4, 5, 6, 7]),
            Some(2)
        );
        assert_eq!(
            Solution::magic_index_distinct(&[-1, 0, 1, 3, 5, 6, 7]),
            Some(3)
        );
        assert_eq!(
            Solution::magic_index_distinct(&[-1, 0, 1, 2, 4, 6, 7]),
            Some(4)
        );
        assert_eq!(
            Solution::magic_index_distinct(&[-1, 0, 1, 2, 3, 5, 7]),
            Some(5)
        );
        assert_eq!(
            Solution::magic_index_distinct(&[-1, 0, 1, 2, 3, 4, 6]),
            Some(6)
        );
    }

    #[test]
    fn finds_magic_index() {
        assert_eq!(
            Solution::magic_index_indistinct(&[4, 4, 4, 4, 4, 4]),
            Some(4)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[4, 4, 4, 4, 6, 6, 6]),
            Some(6)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 2, 2, 2, 2, 2, 4]),
            Some(2)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[0, 3, 4, 6, 8, 9]),
            Some(0)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 3, 4, 6, 6, 6, 6, 6, 8, 8]),
            Some(6)
        );

        // Testing magic index in each position for a even-length array
        assert_eq!(
            Solution::magic_index_indistinct(&[0, 2, 3, 4, 5, 6]),
            Some(0)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 1, 3, 4, 5, 6]),
            Some(1)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 2, 2, 4, 5, 6]),
            Some(2)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 2, 3, 3, 5, 6]),
            Some(3)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 2, 3, 4, 4, 6]),
            Some(4)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 2, 3, 4, 5, 5]),
            Some(5)
        );

        // Testing magic index in each position for a odd-length array
        assert_eq!(
            Solution::magic_index_indistinct(&[0, 2, 3, 4, 5, 6, 7]),
            Some(0)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 1, 3, 4, 5, 6, 7]),
            Some(1)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 2, 2, 4, 5, 6, 7]),
            Some(2)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 2, 3, 3, 5, 6, 7]),
            Some(3)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 2, 3, 4, 4, 6, 7]),
            Some(4)
        );
        assert_eq!(
            Solution::magic_index_indistinct(&[1, 2, 3, 4, 5, 6, 6]),
            Some(6)
        );
    }
}
