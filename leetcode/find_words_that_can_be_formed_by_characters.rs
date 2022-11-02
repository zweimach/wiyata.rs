use std::collections::HashMap;

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut count = 0;
    let mut bag = HashMap::new();
    for w in words.iter() {
        for c in w.chars() {
            if let Some(n) = bag.get_mut(&c) {
                *n += 1;
            } else {
                bag.insert(c, 1);
            }
        }
        for c in chars.chars() {
            if let Some(n) = bag.get_mut(&c) {
                if *n > 0 {
                    *n -= 1;
                }
            }
        }
        if bag.iter().fold(0, |a, (_, c)| a + c) == 0 {
            count += w.len() as i32;
        }
        bag.clear();
    }
    count
}
