#[cfg(test)]
use crate::leetcode::is_subsequence::is_subsequence;

#[test]
fn it_checks_if_string_is_a_subsequence() {
    assert!(is_subsequence("abc".into(), "ahbgdc".into()));
    assert!(!is_subsequence("axc".into(), "ahbgdc".into()));
}
