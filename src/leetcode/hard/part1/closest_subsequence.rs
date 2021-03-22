use std::collections::HashSet;
use std::iter::once;

pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
    let (left, right) = nums.split_at(nums.len() / 2);
    let mut lsums = sums(left);
    let rsums = sums(right);
    lsums.sort();
    rsums.into_iter().map(|x| best_difference(&lsums, goal - x)).min().unwrap_or(goal)
}

fn best_difference(elems: &[i32], x: i32) -> i32 {
    match elems.binary_search(&x) {
        Ok(_) => 0,
        Err(0) => elems[0] - x,
        Err(m) if m == elems.len() => x - *elems.last().unwrap(),
        Err(i) => (elems[i] - x).min(x - elems[i - 1]),
    }
}

fn sums(nums: &[i32]) -> Vec<i32> {
    let mut res: HashSet<_> = once(0).collect();
    for &e in nums {
        let new: Vec<_> = res.iter().map(|&x| x + e).collect();
        res.extend(new);
    }
    res.into_iter().collect()
}