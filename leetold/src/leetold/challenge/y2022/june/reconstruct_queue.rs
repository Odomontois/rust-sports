use std::{cmp::Reverse, collections::BinaryHeap};

pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut q = BinaryHeap::new();
    let mut rest: Vec<_> = people.into_iter().map(|v| [v[0], v[1], v[1]]).collect();
    let mut res = vec![];
    while !q.is_empty() || !rest.is_empty() {
        rest.sort_by_key(|v| v[2] == 0);
        let take = rest.iter().rev().take_while(|v| v[2] == 0).count();
        q.extend(rest.drain(rest.len() - take..).map(|[h, k, _]| Reverse([h, k])));
        let Reverse(p) = q.pop().unwrap();
        for q in &mut rest {
            if q[0] <= p[0] {
                q[2] -= 1
            }
        }
        res.push(p.to_vec())
    }

    res
}
