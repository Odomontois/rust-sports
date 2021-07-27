use std::collections::BTreeSet;

pub fn three_sum_closest_bts(nums: Vec<i32>, target: i32) -> i32 {
    let mut t = BTreeSet::new();
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

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort();
    let mut best = i32::MAX;
    for i in 0..nums.len() {
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < k {
            let s = nums[i] + nums[j] + nums[k];
            if (target - s).abs() < (target - best).abs() {
                best = s
            }
            if s < target {
                j += 1
            } else if s > target {
                k -= 1
            } else {
                return target
            }
        }
    }
    best
}
