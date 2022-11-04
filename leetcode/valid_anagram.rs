use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut bag = HashMap::new();
    for c in s.chars() {
        if let Some(n) = bag.get_mut(&c) {
            *n += 1;
        } else {
            bag.insert(c, 1);
        }
    }
    for c in t.chars() {
        if let Some(n) = bag.get_mut(&c) {
            if *n > 0 {
                *n -= 1;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    bag.iter().fold(0, |a, (_, c)| a + c) == 0
}
