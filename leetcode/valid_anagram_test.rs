#![cfg(test)]
use super::valid_anagram::is_anagram;

#[test]
fn it_checks_if_two_strings_are_anagrams() {
    assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    assert!(!is_anagram("rat".to_string(), "car".to_string()));
}
