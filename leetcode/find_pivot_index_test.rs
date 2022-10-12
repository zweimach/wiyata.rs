#![cfg(test)]
use super::find_pivot_index::pivot_index;

#[test]
fn it_returns_the_leftmost_pivot_index() {
    assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(pivot_index(vec![1, 2, 3]), -1);
    assert_eq!(pivot_index(vec![2, 1, -1]), 0);
}
