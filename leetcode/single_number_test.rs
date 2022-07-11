#[cfg(test)]
use crate::leetcode::single_number::single_number;

#[test]
fn it_returns_the_element_that_only_appears_once() {
    assert_eq!(single_number(vec![2, 2, 1]), 1);
    assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    assert_eq!(single_number(vec![1]), 1);
}
