#![cfg(test)]
use super::isomorphic_strings::is_isomorphic;

#[test]
fn it_checks_if_string_is_isomorphic() {
    assert!(is_isomorphic("egg".into(), "add".into()));
    assert!(!is_isomorphic("foo".into(), "bar".into()));
    assert!(is_isomorphic("paper".into(), "title".into()));
    assert!(!is_isomorphic("badc".into(), "baba".into()));
}
