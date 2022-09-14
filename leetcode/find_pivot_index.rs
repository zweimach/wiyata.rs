pub fn pivot_index(nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        let left: i32 = nums[0..i].iter().sum();
        let right: i32 = nums[(i + 1)..].iter().sum();
        if left == right {
            return i as i32;
        }
    }
    -1
}
