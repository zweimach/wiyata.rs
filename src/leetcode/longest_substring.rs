pub fn length_of_longest_substring(s: String) -> i32 {
    let mut acc: Vec<i32> = vec![];
    let mut buf: Vec<char> = vec![];
    for (_, c) in s.chars().enumerate() {
        if buf.contains(&c) {
            let idx = buf.iter().position(|&i| i == c).unwrap_or(0);
            acc.push(buf.len() as i32);
            buf.drain(0..=idx);
            buf.push(c);
        } else {
            buf.push(c);
        }
    }
    acc.push(buf.len() as i32);
    *acc.iter().max().unwrap_or(&0)
}
