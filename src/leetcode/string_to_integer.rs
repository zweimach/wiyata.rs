pub fn my_atoi(s: String) -> i32 {
    let mut result = 0;
    let mut neg = false;
    let mut overflow = false;
    let mut count = 0;

    for c in s.chars() {
        match c {
            ' ' if count == 0 => continue,
            '+' if count == 0 => count = 1,
            '-' if count == 0 => {
                neg = true;
                count = 1
            }
            c @ '0'..='9' => {
                let n = c.to_string().parse::<i32>().unwrap();
                let can_overflow = i32::checked_mul(result, 10)
                    .map(|m| m.checked_add(n))
                    .flatten();
                if let Some(n) = can_overflow {
                    result = n;
                } else {
                    overflow = true;
                };
                count += 1
            }
            _ => break,
        };
    }
    match (neg, overflow) {
        (true, true) => i32::min_value(),
        (true, false) => result * -1,
        (false, true) => i32::max_value(),
        (false, false) => result,
    }
}
