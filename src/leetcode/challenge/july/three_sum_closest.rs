use std::collections::BTreeSet;
use std::iter::successors;

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
    let f = |(i, &x): (usize, &i32)| {
        let s = successors(Some(&nums[i + 1..]), move |s| {
            if x + s[0] + s[s.len() - 1] < target {
                Some(&s[1..])
            } else {
                Some(&s[..s.len() - 1])
            }
        });
        s.take_while(|s| s.len() > 1).map(move |s| x + s[0] + s[s.len() - 1])
    };
    let ns = nums.iter().enumerate().flat_map(f);
    ns.min_by_key(|&s| (target - s).abs()).unwrap()
}
