pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 1;
    while i < nums.len() {
        if nums[i - 1] == nums[i] {
            nums.remove(i);
        } else {
            i += 1;
        }
    }
    nums.len() as i32
}
