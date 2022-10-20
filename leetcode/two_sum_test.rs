#![cfg(test)]
use super::two_sum::two_sum;

#[test]
fn it_returns_the_indices_that_add_up_to_target() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}
