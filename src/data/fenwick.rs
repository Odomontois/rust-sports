pub struct FenSum<A> {
    items: Vec<A>,
    cap: usize,
}

impl<A: Copy> FenSum<A> {
    pub fn new(init: A, cap: usize) -> Self {
        FenSum {
            items: vec![init; cap * 4],
            cap,
        }
    }
}

impl<A: Copy + AddAssign> FenSum<A> {
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

use std::{
    borrow::Borrow,
    ops::{Add, AddAssign, RangeBounds},
};
use std::{
    ops::{Bound::*, Range},
    usize,
};

impl<A: Copy + Add<Output = A>> FenSum<A> {
    pub fn get<I: RangeBounds<usize>>(&self, index: I) -> Option<A> {
        let from = match index.start_bound() {
            Unbounded => 0,
            Excluded(&x) => x + 1,
            Included(&x) => x,
        };
        let to = match index.end_bound() {
            Unbounded => self.cap,
            Excluded(&x) => x,
            Included(&x) => x.max(1) - 1,
        };
        self.get_iter(from, to, 0, 0, self.cap)
    }

    fn get_iter(&self, from: usize, to: usize, p: usize, l: usize, r: usize) -> Option<A> {
        if l >= from && r <= to {
            return Some(self.items[p]);
        }
        if l >= to || r <= from {
            return None;
        }
        let m = (l + r + 1) / 2;
        self.get_iter(from, to, 2 * p + 1, l, m)
            .into_iter()
            .chain(self.get_iter(from, to, 2 * p + 2, m, r))
            .fold(None, |opt, y| opt.map(|x| x + y).or(Some(y)))
    }
}

pub struct Fen<A, B, T, F> {
    elems: Vec<A>,
    agg: Vec<B>,
    trans: T,
    combine: F,
}
use super::segpos::SegPos;

impl<A, B: Clone, T, F> Fen<A, B, T, F>
where
    T: Fn(&A) -> B,
    F: Fn(B, B) -> B,
{
    pub fn new(elems: Vec<A>, trans: T, combine: F) -> Self {
        let agg = Vec::with_capacity(elems.len() - 1);
        let mut result = Self {
            elems,
            agg,
            trans,
            combine,
        };

        result.init(SegPos::start(result.elems.len()));
        result
    }

    fn init(&mut self, pos: SegPos) -> B {
        if pos.elem() {
            return (self.trans)(&self.elems[pos.from]);
        }
        let (lp, rp) = pos.subs();
        let l = self.init(lp);
        let r = self.init(rp);
        let x = (self.combine)(l, r);
        self.agg.push(x.clone());
        x
    }

    fn calc(&self, rng: Range<usize>) -> B {
        self.range_iter(SegPos::start(self.elems.len()), &rng)
    }

    fn range_iter(&self, pos: SegPos, rng: &Range<usize>) -> B {
        if pos.elem() {
            return (self.trans)(&self.elems[pos.from]);
        }
        if pos.inside(rng) {
            return self.agg[pos.p].clone();
        }
        let (l, r) = pos.subs();
        if !l.intersects(rng) {
            self.range_iter(r, rng)
        } else if !r.intersects(rng) {
            self.range_iter(l, rng)
        } else {
            (self.combine)(self.range_iter(l, rng), self.range_iter(r, rng))
        }
    }
}

struct MutElem<'a, A, B, F, T> {
    fen: &'a mut Fen<A, B, F, T>,
    idx: usize,
}

impl<'a, A, B, F, T> Borrow<&'a mut A> for MutElem<'a, A, B, F, T> {
    fn borrow<'b>(&'b self) -> &'b &'a mut A {
        let x: &'a mut A = &mut self.fen.elems[self.idx];
        &x
    }
}

// impl<A, B, F, T> AsRef<A> for MutElem<A, B, F, T> {
//     fn borrow_mut(&mut self) -> &mut A {
//         &mut self.fen.elems[self.idx]
//     }
// }
