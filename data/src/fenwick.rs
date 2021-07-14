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
    borrow::{Borrow, Cow},
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

impl<A, B, T, F> Fen<A, B, T, F>
where
    T: Fn(&A) -> B,
    F: Fn(&B, &B) -> B,
{
}

impl<A, B: Clone, T, F> Fen<A, B, T, F>
where
    T: Fn(&A) -> B,
    F: Fn(&B, &B) -> B,
{
    pub fn new(elems: Vec<A>, trans: T, combine: F) -> Self {
        let agg = Vec::with_capacity(elems.len() - 1);
        let mut result = Self {
            elems,
            agg,
            trans,
            combine,
        };

        result.init(result.start()).ok();
        result
    }

    pub fn calc(&self, rng: Range<usize>) -> B {
        self.range_iter(self.start(), &rng).into_owned()
    }

    pub fn get(&mut self, idx: usize) -> Option<MutElem<A, B, T, F>> {
        if idx < self.elems.len() {
            return None;
        }
        Some(MutElem { fen: self, idx })
    }

    fn init_val(&self, r: Result<usize, usize>) -> Cow<B> {
        match r {
            Err(el) => Cow::Owned((self.trans)(&self.elems[el])),
            Ok(ag) => Cow::Borrowed(&self.agg[ag]),
        }
    }

    fn init(&mut self, pos: SegPos) -> Result<usize, usize> {
        if pos.elem() {
            return Err(pos.from);
        }
        let (lp, rp) = pos.subs();
        let l = self.init(lp);
        let r = self.init(rp);
        let x = (self.combine)(self.init_val(l).borrow(), self.init_val(r).borrow());
        self.agg.push(x);
        Ok(self.agg.len() - 1)
    }

    fn start(&self) -> SegPos {
        SegPos::start(self.elems.len())
    }

    fn fix(&mut self, index: usize) {
        self.fix_iter(index, self.start())
    }

    fn agg_value(&self, pos: SegPos) -> Cow<B> {
        if pos.elem() {
            Cow::Owned((self.trans)(&self.elems[pos.from]))
        } else {
            Cow::Borrowed(&self.agg[pos.p])
        }
    }

    fn fix_iter(&mut self, index: usize, pos: SegPos) {
        if pos.elem() {
            return;
        }
        let (l, r) = pos.subs();
        if l.contains(index) {
            self.fix_iter(index, l)
        } else if r.contains(index) {
            self.fix_iter(index, r)
        } else {
            return;
        }
        let res = (self.combine)(self.agg_value(l).borrow(), self.agg_value(r).borrow());
        self.agg[pos.p] = res;
    }

    fn range_iter(&self, pos: SegPos, rng: &Range<usize>) -> Cow<B> {
        if pos.elem() {
            return Cow::Owned((self.trans)(&self.elems[pos.from]));
        }
        if pos.inside(rng) {
            return Cow::Borrowed(&self.agg[pos.p]);
        }
        let (l, r) = pos.subs();
        if !l.intersects(rng) {
            self.range_iter(r, rng)
        } else if !r.intersects(rng) {
            self.range_iter(l, rng)
        } else {
            Cow::Owned((self.combine)(
                self.range_iter(l, rng).borrow(),
                self.range_iter(r, rng).borrow(),
            ))
        }
    }
}

pub struct MutElem<'a, A, B: Clone, T, F>
where
    T: Fn(&A) -> B,
    F: Fn(&B, &B) -> B,
{
    fen: &'a mut Fen<A, B, T, F>,
    idx: usize,
}

impl<'a, A, B: Clone, F, T> AsRef<A> for MutElem<'a, A, B, T, F>
where
    T: Fn(&A) -> B,
    F: Fn(&B, &B) -> B,
{
    fn as_ref(&self) -> &A {
        &self.fen.elems[self.idx]
    }
}

impl<'a, A, B: Clone, F, T> AsMut<A> for MutElem<'a, A, B, T, F>
where
    T: Fn(&A) -> B,
    F: Fn(&B, &B) -> B,
{
    fn as_mut(&mut self) -> &mut A {
        &mut self.fen.elems[self.idx]
    }
}

impl<'a, A, B: Clone, F, T> Drop for MutElem<'a, A, B, T, F>
where
    T: Fn(&A) -> B,
    F: Fn(&B, &B) -> B,
{
    fn drop(&mut self) {
        self.fen.fix(self.idx)
    }
}
