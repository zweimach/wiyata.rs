pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let mut s_iter = s.chars().peekable();
    while let Some(i) = s_iter.next() {
        match i {
            'I' => match s_iter.peek() {
                Some(&'V' | &'X') => result -= 1,
                _ => result += 1,
            },
            'V' => result += 5,
            'X' => match s_iter.peek() {
                Some(&'L' | &'C') => result -= 10,
                _ => result += 10,
            },
            'L' => result += 50,
            'C' => match s_iter.peek() {
                Some(&'D' | &'M') => result -= 100,
                _ => result += 100,
            },
            'D' => result += 500,
            'M' => result += 1000,
            _ => unreachable!(),
        };
    }
    result
}
