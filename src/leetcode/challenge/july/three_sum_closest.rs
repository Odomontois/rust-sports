use std::collections::BTreeSet;

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let mut t = BTreeSet::new();
    nums.sort();
    (0..nums.len())
        .filter_map(move |i| {
            let x = nums[i];
            let best = nums[i + 1..]
                .iter()
                .flat_map(|&y| {
                    let q = target - x - y;
                    let from = t.range(q..).next();
                    let to = t.range(..q).rev().next();
                    from.into_iter().chain(to).copied().map(move |z| x + y + z)
                })
                .min_by_key(|s| (s - target).abs());
            t.insert(x);
            best
        })
        .min_by_key(|s| (s - target).abs())
        .unwrap_or(-1)
}
