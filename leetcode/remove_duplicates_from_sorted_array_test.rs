#![cfg(test)]
use super::remove_duplicates_from_sorted_array::remove_duplicates;

#[test]
fn it_returns_the_number_of_unique_elements() {
    assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
    assert_eq!(
        remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
        5
    );
}
