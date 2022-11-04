pub fn reverse_vowels(s: String) -> String {
    let mut result = String::with_capacity(s.len());
    let mut i = s.chars();
    let mut j = s.chars();
    while let Some(c) = i.next() {
        if !check(c) {
            result.push(c);
            continue;
        }
        while let Some(v) = j.next_back() {
            if check(v) {
                result.push(v);
                break;
            }
        }
    }
    result
}

fn check(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'i' | 'u' | 'e' | 'o' => true,
        _ => false,
    }
}
