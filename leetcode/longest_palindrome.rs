use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> i32 {
    let mut m = HashMap::new();
    for i in s.chars() {
        if let Some(&n) = m.get(&i) {
            m.insert(i, n + 1);
        } else {
            m.insert(i, 1);
        }
    }
    let has_odd = m.values().any(|&v| v % 2 != 0);
    m.values().fold(0, |a, &c| a + c / 2) * 2 + if has_odd { 1 } else { 0 }
}
