pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
    let corect = nums.iter().all(|&x| (threshold + x - 1) / x > nums.len() as i32);
    if corect {nums.len() as i32} else {-1}
}