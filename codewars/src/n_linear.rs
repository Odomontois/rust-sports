use std::{cmp::Reverse, collections::BinaryHeap, iter::from_fn};

fn n_linear(m: &[u32], n: usize) -> u32 {
    let mut q = BinaryHeap::new();
    let mut prev = 0;
    q.push(Reverse(1u64));
    let mut is = from_fn(|| {
        let mut cur = prev;
        while cur == prev {
            cur = q.pop()?.0;
        }
        prev = cur;
        for &k in m {
            q.push(Reverse(k as u64 * cur + 1))
        }
        Some(cur)
    });

    is.nth(n).unwrap_or(0) as u32
}
