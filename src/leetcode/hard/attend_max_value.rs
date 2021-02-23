use std::{cmp::Reverse, collections::BinaryHeap};

use crate::data::ignore::Ign;

pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut events: Vec<_> = events.into_iter().map(|v| [v[0], v[1], v[2]]).collect();
    events.sort_by_key(|&[s, _, _]| s);
    MaxValue::new(k as usize).walk(events)
}

struct MaxValue {
    k: usize,
    cur: Vec<i32>,
    q: BinaryHeap<(Reverse<i32>, Ign<Vec<i32>>)>,
}
impl MaxValue {
    fn new(k: usize) -> Self {
        Self {
            k,
            cur: vec![0; k],
            q: BinaryHeap::new(),
        }
    }

    fn add(&mut self, s: i32) {
        while self.q.peek().filter(|(t, _)| t.0 < s).is_some() {
            if let Some((_, Ign(res))) = self.q.pop() {
                for i in 0..self.k {
                    self.cur[i] = self.cur[i].max(res[i]);
                }
            }
        }
    }

    fn walk(&mut self, events: impl IntoIterator<Item = [i32; 3]>) -> i32 {
        for [s, e, v] in events {
            self.add(s);
            let mut res = vec![0; self.k];
            for i in 0..self.k {
                res[i] = v + if i == 0 { 0 } else { self.cur[i - 1] };
            }
            self.q.push((Reverse(e), Ign(res)));
        }
        self.add(std::i32::MAX);
        self.cur.iter().copied().max().unwrap_or(0)
    }
}

#[test]
fn test_walk() {
    fn check(xs: &[[i32; 3]], k: usize, exp: i32) {
        assert_eq!(max_value(xs.iter().map(|v| v.to_vec()).collect(), k as i32), exp)
    }
    check(&[[1,2,4],[3,4,3],[2,3,1]], 2, 7);
    check(&[[1,2,4],[3,4,3],[2,3,10]], 2, 10);
    check(&[[1,1,1],[2,2,2],[3,3,3],[4,4,4]], 3, 9);
}
