pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut t = Vec::new();
    let min = strs.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap();
    for i in strs.iter() {
        let mut n = 0;
        while n < min.len() && min[0..=n] == i[0..=n] {
            n += 1;
        }
        t.push(n);
    }
    let &n = t.iter().min().unwrap_or(&0);
    min[0..n].into()
}
