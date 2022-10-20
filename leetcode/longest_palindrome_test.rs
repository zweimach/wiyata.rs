#![cfg(test)]
use super::longest_palindrome::longest_palindrome;

#[test]
fn it_returns_the_length_of_longest_palindrome() {
    assert_eq!(longest_palindrome("abccccdd".to_string()), 7);
    assert_eq!(longest_palindrome("a".to_string()), 1);
}
