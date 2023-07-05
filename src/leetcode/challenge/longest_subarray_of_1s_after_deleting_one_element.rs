pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold([0, -1, 0], |[best, prev, cur], x| {
        if x == 1 {
            [best.max(prev + cur + 1), prev, cur + 1]
        } else {
            [best.max(cur), cur, 0]
        }
    })[0]
}
