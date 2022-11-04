#![cfg(test)]
use super::flood_fill::flood_fill;

#[test]
fn it_performs_flood_fill_on_the_input_matrix() {
    let input1 = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    let output1 = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
    assert_eq!(flood_fill(input1, 1, 1, 2), output1);

    let input2 = vec![vec![0, 0, 0], vec![0, 0, 0]];
    let output2 = vec![vec![0, 0, 0], vec![0, 0, 0]];
    assert_eq!(flood_fill(input2, 0, 0, 0), output2);
}
