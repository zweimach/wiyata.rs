#[cfg(test)]
use wiyata::leetcode::richest_customer_wealth::maximum_wealth;

#[test]
fn it_returns_max_customer_wealth() {
    assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6);
    assert_eq!(maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10);
    assert_eq!(
        maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
        17
    );
}
