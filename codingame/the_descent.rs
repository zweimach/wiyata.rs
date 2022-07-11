pub fn the_descent(values: &[i32]) -> usize {
    let max_value = values.iter().fold(0i32, |mut max, &val| {
        if val > max {
            max = val
        }
        max
    });
    values.iter().position(|&r| r == max_value).unwrap()
}
