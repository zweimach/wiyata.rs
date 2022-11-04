#![cfg(test)]
use super::remove_element::remove_element;

#[test]
fn it_removes_all_occurrences_of_target_element() {
    let mut input1 = vec![3, 2, 2, 3];
    let output1 = vec![2, 2];
    assert_eq!(remove_element(&mut input1, 3), 2);
    assert_eq!(input1, output1);

    let mut input2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let output2 = vec![0, 1, 4, 0, 3];
    assert_eq!(remove_element(&mut input2, 2), 5);
    assert_eq!(input2, output2);
}
