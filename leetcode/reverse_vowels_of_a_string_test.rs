#![cfg(test)]
use super::reverse_vowels_of_a_string::reverse_vowels;

#[test]
fn it_reverses_all_the_vowels_in_the_string() {
    assert_eq!(reverse_vowels("hello".to_string()), "holle");
    assert_eq!(reverse_vowels("leetcode".to_string()), "leotcede");
}
