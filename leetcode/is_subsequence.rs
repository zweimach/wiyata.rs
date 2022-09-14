use std::collections::VecDeque;

pub fn is_subsequence(s: String, t: String) -> bool {
    let mut q = s.chars().collect::<VecDeque<_>>();
    for i in t.chars() {
        if let Some(&c) = q.front() {
            if c == i {
                q.pop_front();
            }
        } else {
            break;
        }
    }
    q.is_empty()
}
