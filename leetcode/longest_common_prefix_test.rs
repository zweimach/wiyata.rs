#![cfg(test)]
use super::longest_common_prefix::longest_common_prefix;

#[test]
fn it_returns_the_longest_common_prefix() {
    assert_eq!(
        longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]),
        "fl"
    );
    assert_eq!(
        longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]),
        ""
    );
}
