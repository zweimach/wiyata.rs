pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    'block: for (i, &x) in nums.iter().enumerate() {
        for (j, &y) in nums.iter().enumerate().skip(i + 1) {
            if x + y == target {
                result.push(i as i32);
                result.push(j as i32);
                break 'block;
            }
        }
    }
    result
}
