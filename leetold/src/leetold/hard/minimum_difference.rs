// https://leetcode.com/problems/minimum-difference-in-sums-after-removal-of-elements
use std::{cmp::Reverse, collections::BinaryHeap};

pub fn minimum_difference(nums: Vec<i32>) -> i64 {
    let n = nums.len() / 3;
    let ni = nums.iter().copied().map(<i32>::into);
    let r2l: Vec<_> = chooses(ni.clone().rev(), n, Reverse, |x| x.0).collect();
    let l2r = chooses(ni, n, |x| x, |x| x);
    l2r.zip(r2l.into_iter().rev()).map(|(f, s)| f - s).min().unwrap_or(0)
}

pub fn chooses<A: Ord + 'static>(
    mut xs: impl Iterator<Item = i64>,
    n: usize,
    to: impl Fn(i64) -> A + 'static,
    from: impl Fn(A) -> i64 + Copy + 'static,
) -> impl Iterator<Item = i64> {
    let mut heap = BinaryHeap::new();
    let mut s: i64 = xs.by_ref().take(n).inspect(|&x| heap.push(to(x))).sum();

    Some(s).into_iter().chain(xs.take(n).map(move |x| {
        s += x;
        heap.push(to(x));
        s -= heap.pop().map(from).unwrap_or(0);
        s
    }))
}
