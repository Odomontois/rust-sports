use std::{cmp::Reverse, collections::BinaryHeap};

pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let candidates = candidates as usize;
    let mut costs = costs.into_iter().enumerate().map(|(i, x)| Reverse((x, i)));
    let mut heap = BinaryHeap::with_capacity(candidates * 2);
    heap.extend(costs.by_ref().take(candidates).map(|x| (x, true)).collect());
    heap.extend(costs.by_ref().rev().map(|x| (x, false)).take(candidates));

    (0..k)
        .map(|_| {
            let (Reverse((cost, _)), init) = heap.pop().unwrap();
            heap.extend(if init {
                costs.next().map(|x| (x, true))
            } else {
                costs.next_back().map(|x| (x, false))
            });
            cost as i64
        })
        .sum()
}
