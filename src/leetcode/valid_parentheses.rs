pub fn is_valid(xs: String) -> bool {
    fn f(c: char) -> char {
        match c {
            ']' => '[',
            '}' => '{',
            ')' => '(',
            _ => '\0',
        }
    }
    let stack = xs
        .chars()
        .fold(vec![], |mut stack, curr| match stack.last() {
            Some(c) if *c == f(curr) => {
                stack.pop();
                stack
            }
            _ => {
                stack.push(curr);
                stack
            }
        });
    stack.len() == 0
}
