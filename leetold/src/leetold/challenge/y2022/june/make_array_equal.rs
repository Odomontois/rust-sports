// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/

pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let n64 = || nums.iter().map(|x| *x as i64);
    let n = nums.len() as i64;
    let sum: i64 = n64().sum();
    n64()
        .enumerate()
        .scan(0, |s, (i, x)| {
            let res = (2 * i as i64 - n) * x - 2 * *s + sum;
            *s += x;
            Some(res)
        })
        .min()
        .unwrap_or(0) as i32
}
