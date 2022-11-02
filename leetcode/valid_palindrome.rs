pub fn is_palindrome(s: String) -> bool {
    let mut i = s.chars();
    while let Some(c) = i.next() {
        if !c.is_alphanumeric() {
            continue;
        }
        while let Some(v) = i.next_back() {
            if !v.is_alphanumeric() {
                continue;
            }
            if c.to_ascii_lowercase() != v.to_ascii_lowercase() {
                return false;
            }
            break;
        }
    }
    true
}
