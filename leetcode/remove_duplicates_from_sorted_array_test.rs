#![cfg(test)]
use super::remove_duplicates_from_sorted_array::remove_duplicates;

#[test]
fn it_removes_duplicated_elements() {
    let mut input1 = vec![1, 1, 2];
    let output1 = vec![1, 2];
    assert_eq!(remove_duplicates(&mut input1), 2);
    assert_eq!(input1, output1);

    let mut input2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let output2 = vec![0, 1, 2, 3, 4];
    assert_eq!(remove_duplicates(&mut input2), 5);
    assert_eq!(input2, output2);
}
