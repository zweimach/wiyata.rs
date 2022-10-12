#![cfg(test)]
use super::utf_8_validation::valid_utf8;

#[test]
fn it_checks_for_valid_utf_8_byte_sequence() {
    assert!(valid_utf8(vec![197, 130, 1]));
    assert!(!valid_utf8(vec![235, 140, 4]));
}
