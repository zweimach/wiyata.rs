#[cfg(test)]
use wiyata::codingame::the_descent::the_descent;

#[test]
fn it_returns_max_value_index() {
    assert_eq!(the_descent(&[10, 12, 19, 14, 20, 0, 13, 15]), 4);
    assert_eq!(the_descent(&[0, 1, 2, 3, 4, 5, 6, 7]), 7);
}
