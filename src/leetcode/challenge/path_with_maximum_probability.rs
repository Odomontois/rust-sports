use std::{
    collections::{BinaryHeap},
    convert::TryInto,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct F64(f64);

impl Eq for F64 {}
impl Ord for F64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
    let (n, start, end) = (n as usize, start as usize, end as usize);
    let mut adj = vec![(vec![]); n];
    for (cost, edge) in succ_prob.into_iter().zip(edges) {
        let t: Result<[i32; 2], _> = edge.try_into();
        if let Ok(p) = t {
            let [i, j] = p.map(|x| x as usize);
            adj[i].push((j, cost));
            adj[j].push((i, cost));
        }
    }
    let mut costs = vec![0.0; n];
    costs[start] = 1.0;
    let mut q: BinaryHeap<_> = [(F64(1.0), start)].iter().copied().collect();
    while let Some((F64(cost), i)) = q.pop() {
        if cost < costs[i] {
            continue;
        }
        if i == end {
            return cost;
        }
        for &(j, c) in adj[i].iter() {
            let new_cost = cost * c;
            if new_cost > costs[j] {
                costs[j] = new_cost;
                q.push((F64(new_cost), j));
            }
        }
    }

    0.0
}
