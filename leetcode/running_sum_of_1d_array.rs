pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    (0..nums.len()).map(|i| nums[0..=i].iter().sum()).collect()
}
