#![cfg(test)]
use super::valid_palindrome_ii::valid_palindrome;

#[test]
fn it_checks_if_a_string_is_a_valid_palindrome() {
    assert!(valid_palindrome("eedede".to_string()));
    assert!(valid_palindrome("aba".to_string()));
    assert!(valid_palindrome("abca".to_string()));
    assert!(valid_palindrome("abcba".to_string()));
    assert!(valid_palindrome("acbba".to_string()));
    assert!(valid_palindrome("abbca".to_string()));
    assert!(!valid_palindrome("abc".to_string()));
    assert!(valid_palindrome("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga".to_string()));
}
