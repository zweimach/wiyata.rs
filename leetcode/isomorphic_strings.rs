use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    check(&s, &t) && check(&t, &s)
}

fn check(s: &str, t: &str) -> bool {
    let mut m = HashMap::new();
    let l = s.chars().zip(t.chars());
    for (s, t) in l {
        if let Some(&d) = m.get(&s) {
            if t != d {
                return false;
            }
        } else {
            m.insert(s, t);
        }
    }
    true
}
