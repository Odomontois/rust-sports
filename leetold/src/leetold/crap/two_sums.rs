pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, a) in nums.iter().enumerate() {
            for (j, b) in (&nums[i + 1..]).iter().enumerate() {
                if a + b == target {
                    return vec![i as i32, (i + j + 1) as i32];
                }
            }
        }
        vec![]
    }
}