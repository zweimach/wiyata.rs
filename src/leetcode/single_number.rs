pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |a, c| a ^ c)
}
