pub struct Fen<A> { items: Vec<A>, cap: usize }

impl<A: Copy> Fen<A> {
    pub fn new(init: A, cap: usize) -> Self {
        Fen { items: vec![init; cap * 3], cap }
    }
}

impl<A: Copy + AddAssign> Fen<A> {
    pub fn increment(&mut self, idx: usize, item: A) {
        let (mut p, mut l, mut r) = (0, 0, self.cap);
        while l + 1 < r {
            self.items[p] += item;
            let m = (l + r + 1) / 2;
            if idx < m {
                p = 2 * p + 1;
                r = m;
            } else {
                p = 2 * p + 2;
                l = m;
            }
        }
        self.items[p] += item;
    }
}

use std::ops::Bound::*;
use std::ops::{AddAssign, Add, RangeBounds};

impl<A: Copy + Add<Output=A>> Fen<A> {
    pub fn get<I: RangeBounds<usize>>(&self, index: I) -> Option<A> {
        let from = match index.start_bound() {
            Unbounded => 0,
            Excluded(&x) => x + 1,
            Included(&x) => x
        };
        let to = match index.end_bound() {
            Unbounded => self.cap,
            Excluded(&x) => x,
            Included(&x) => x.max(1) - 1
        };
        self.get_iter(from, to, 0, 0, self.cap)
    }
    fn get_iter(&self, from: usize, to: usize, p: usize, l: usize, r: usize) -> Option<A> {
        if l >= from && r <= to { return Some(self.items[p]); }
        if l >= to || r <= from { return None; }
        let m = (l + r + 1) / 2;
        self.get_iter(from, to, 2 * p + 1, l, m)
            .into_iter()
            .chain(self.get_iter(from, to, 2 * p + 2, m, r))
            .fold(None, |opt, y| opt.map(|x| x + y).or(Some(y)))
    }
}