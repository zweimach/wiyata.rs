pub fn valid_palindrome(s: String) -> bool {
    check(&s) || check(&s.chars().rev().collect())
}

fn check(s: &String) -> bool {
    let mut m = 1;
    let mut i = s.chars().peekable();
    'front: while let Some(c) = i.next() {
        while let Some(v) = i.next_back() {
            if c == v {
                break;
            }
            if m == 0 {
                return false;
            }
            m -= 1;
            if let Some(c) = i.peek() {
                if *c == v {
                    i.next();
                    continue 'front;
                }
            }
        }
    }
    true
}
