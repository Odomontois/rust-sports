pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let l = nums.len() - (k as usize) % nums.len();
    nums[..l].reverse();
    nums[l..].reverse();
    nums.reverse()
}