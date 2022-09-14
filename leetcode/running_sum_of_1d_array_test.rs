#[cfg(test)]
use crate::leetcode::running_sum_of_1d_array::running_sum;

#[test]
fn it_returns_the_element_that_only_appears_once() {
    assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
    assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
}
