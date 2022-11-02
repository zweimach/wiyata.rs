#![cfg(test)]
use super::valid_palindrome::is_palindrome;

#[test]
fn it_checks_if_a_string_is_a_valid_palindrome() {
    assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    assert!(!is_palindrome("race a car".to_string()));
    assert!(is_palindrome(" ".to_string()));
    assert!(is_palindrome("race a E car".to_string()));
}
