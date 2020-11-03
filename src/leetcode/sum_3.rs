pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = nums.iter().cloned().enumerate().flat_map(|(i, a)| {
            (&nums[i + 1..]).iter().cloned().enumerate().map(move |(j, b)| (i + j + 1, a, b))
        }).flat_map(|(j, a, b)| {
            (&nums[j + 1..]).iter().cloned().filter_map(move |c| {
                if a + b + c == 0 {
                    let mut res = vec![a, b, c];
                    res.sort();
                    Some(res)
                } else { None }
            })
        }).collect();
        res.sort();
        res.dedup();
        res
    }
}