pub fn find_min(nums: Vec<i32>) -> i32 {
    if nums[0] < nums[nums.len() - 1] {
        return nums[0];
    }
    let (mut start, mut end) = (0, nums.len() - 1);
    while start + 1 < end {
        let m = (start + end) / 2;
        if nums[m] > nums[start] {
            start = m
        } else {
            end = m
        }
    }
    nums[end]
}
