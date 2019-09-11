#[cfg(test)]
use crate::codingame::onboarding::onboarding;

#[test]
fn it_returns_min_value() {
    assert_eq!(onboarding(&("e1", 12), &("e2", 10)), "e2");
    assert_eq!(onboarding(&("e3", 15), &("e4", 20)), "e3");
}
