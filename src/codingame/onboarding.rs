pub fn onboarding<'a>(
    (enemy1, dist1): &(&'a str, i32),
    (enemy2, dist2): &(&'a str, i32),
) -> &'a str {
    if dist1 < dist2 {
        enemy1
    } else {
        enemy2
    }
}
