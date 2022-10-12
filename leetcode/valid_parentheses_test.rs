#![cfg(test)]
use super::valid_parentheses::is_valid;

#[test]
fn it_returns_true_when_brackets_match() {
    assert_eq!(is_valid("(){}[]".to_string()), true);
    assert_eq!(is_valid("[({(()()())})]".to_string()), true);
    assert_eq!(is_valid("{([({[]}[])()])}".to_string()), true);
}

#[test]
fn it_returns_false_when_brackets_do_not_match() {
    assert_eq!(is_valid("([{({[]})}]))".to_string()), false);
    assert_eq!(is_valid("[](){}(){}[]]]]".to_string()), false);
    assert_eq!(is_valid("{{[[(((".to_string()), false);
}
