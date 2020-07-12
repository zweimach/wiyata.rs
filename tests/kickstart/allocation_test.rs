#[cfg(test)]
use wiyata::kickstart::allocation::allocation;

#[test]
fn it_returns_max_houses_to_buy() {
    assert_eq!(allocation(&vec![20, 40, 90, 90], 100), 2);
    assert_eq!(allocation(&vec![10, 10, 30, 30], 50), 3);
    assert_eq!(allocation(&vec![999, 999, 999], 300), 0);
}
