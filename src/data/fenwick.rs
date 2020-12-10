use std::convert::TryInto;
use std::iter::{FromIterator, repeat};
use std::ops::{Add, Bound, RangeBounds, AddAssign, Mul};

pub trait Monoid: Clone + Add<Self, Output=Self> + AddAssign {
    fn zero() -> Self;
}

impl Monoid for i32 { fn zero() -> Self { 0 } }

impl Monoid for i64 { fn zero() -> Self { 0 } }

pub trait Measured<M> {
    fn measure(&self) -> M;
}

impl Measured<i32> for i32 { fn measure(&self) -> Self { *self } }

impl Measured<i64> for i64 { fn measure(&self) -> Self { *self } }

pub trait Rig: Monoid + Mul<Self, Output=Self> {
    fn one() -> Self;
}

impl Rig for i32 { fn one() -> Self { 1 } }

impl Rig for i64 { fn one() -> Self { 1 } }


#[derive(Debug, Clone)]
struct Fen<A, M> { measures: Vec<M>, values: Vec<A>, add: Option<Vec<M>>, cap: usize }

impl<A, M> Fen<A, M> {
    fn new() -> Self {
        Fen { values: Vec::new(), measures: Vec::new(), add: None, cap: 1 }
    }
}

fn grow_bin<A: Clone>(items: &mut Vec<A>, root: A, zero: A) {
    let mut res = vec![root];
    let mut i = 1;
    let size = items.len() + 1;
    {
        let mut it = items.drain(..);
        while i < size {
            res.extend(it.by_ref().take(i));
            res.extend(repeat(zero.clone()).take(i));
            i *= 2;
        }
    }
    std::mem::swap(items, &mut res);
}

impl<A, M: Monoid> Fen<A, M> where A: Measured<M> {
    pub fn range_calc(&self, from: usize, to: usize) -> M {
        self.range_iter(0, 0, self.cap, from, to)
    }

    fn check_extend(&mut self) {
        if self.values.len() < self.cap { return; }
        let m = self.measure();
        self.cap *= 2;
        grow_bin(&mut self.measures, m, M::zero());
    }

    pub fn push(&mut self, a: A) {
        self.check_extend();
        let mut i = self.values.len();
        let m = a.measure();
        self.values.push(a);
        let mut c = self.cap / 2;
        let mut cur = 0;
        while cur < self.measures.len() {
            self.measures[cur] += m.clone();
            cur = if i < c { 2 * cur + 1 } else {
                i -= c;
                2 * cur + 2
            };
            c /= 2;
        }
    }

    fn range_iter(&self, cur: usize, start: usize, end: usize, from: usize, to: usize) -> M {
        if start >= from && end <= to {
            return if end - start <= 1 { self.values[start].measure() } else { self.measures[cur].clone() };
        }
        if start >= to || end <= from { return M::zero(); }
        let mid = (start + end) / 2;
        let l = self.range_iter(cur * 2 + 1, start, mid, from, to);
        let r = self.range_iter(cur * 2 + 2, mid, end, from, to);
        l + r
    }

    fn bounds<R: UsizeBounds<I>, I>(&self, index: R) -> (usize, usize) {
        index.usize_bounds(self.values.len())
    }

    pub fn calc<R: UsizeBounds<I>, I>(&self, index: R) -> M where {
        let (left, right) = self.bounds(index);
        self.range_calc(left, right)
    }
}

impl<A: Measured<M>, M: Rig> Fen<A, M> {
    fn late_upgrade(&mut self) {
        if let Some(add) = &mut self.add {
            while add.len() < self.cap { grow_bin(add, M::one(), M::one()) }
        } else {
            self.add = Some(vec![M::one(); self.cap * 2 - 1])
        }
    }
    #[allow(unused_variables)]
    pub fn update<R: UsizeBounds<I>, I>(&mut self, index: R, m: M){
        self.late_upgrade();
        let (left, right) = self.bounds(index);
    }
}

impl<'a, A, M> IntoIterator for &'a Fen<A, M> {
    type Item = &'a A;
    type IntoIter = core::slice::Iter<'a, A>;

    fn into_iter(self) -> Self::IntoIter { self.values.iter() }
}

impl<A: Measured<M>, M: Monoid + Clone> FromIterator<A> for Fen<A, M> {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        let mut res = Fen::new();
        for x in iter { res.push(x) }
        res
    }
}

impl<A: Measured<M>, M: Monoid + Clone> Measured<M> for Fen<A, M> {
    fn measure(&self) -> M {
        self.measures.first().cloned()
            .or_else(|| self.values.first().map(|a| a.measure()))
            .unwrap_or_else(|| M::zero())
    }
}

trait UsizeBounds<I> {
    fn usize_bounds(self, len: usize) -> (usize, usize);
}

impl <R: RangeBounds<I>, I: TryInto<usize> + Copy> UsizeBounds<I> for R{
    fn usize_bounds(self, len: usize) -> (usize, usize) {
        let get_bound = |b: Bound<&I>, inc: usize, exc: usize| match b {
            Bound::Unbounded => None,
            Bound::Included(i) => (*i).try_into().ok().map(|x| x + inc),
            Bound::Excluded(i) => (*i).try_into().ok().map(|x| x + exc)
        };
        let left = get_bound(self.start_bound(), 0, 1).unwrap_or(0);
        let right = get_bound(self.end_bound(), 1, 0).unwrap_or(len);
        (left, right)
    }
}


#[cfg(test)]
mod test {
    use std::env;
    use std::str::FromStr;

    use rand::{Rng, thread_rng};

    use super::*;

    fn parse_env<A: FromStr>(name: &str, default: A) -> A {
        env::var(name).ok().and_then(|s| s.parse().ok()).unwrap_or(default)
    }

    #[test]
    fn test_fen_small() {
        let mut f: Fen<i64, i64> = Fen::new();
        for i in 1..100i64 {
            println!("{:?} ", f);
            f.push(i);
        }
    }

    #[test]
    fn test_fen() {
        let mut rnd = thread_rng();
        let size: i64 = parse_env("FENWICK_SIZE", 10_000_000);
        let count: i64 = parse_env("FENWICK_COUNT", 10);
        let f: Fen<i64, i64> = (0..=size).collect();
        for _ in 1..count {
            let i = rnd.gen_range(0, size);
            let j = rnd.gen_range(0, size);
            let res = if j <= i { 0 } else { (i + j - 1) * (j - i) / 2 };
            assert_eq!(f.calc(i..j), res);
        }
        println!("{} {}", size, count);
    }
}


