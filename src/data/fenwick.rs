use std::convert::TryInto;
use std::iter::{FromIterator};
use std::ops::{Add, Bound, RangeBounds};

trait RangeCalc {
    type Output;
    fn range_calc(&self, from: usize, to: usize) -> Self::Output;
}

#[derive(Clone, Debug)]
struct Bin<A, M> { measures: Vec<M>, values: Vec<A>, level: usize }

impl<A: Measured<M>, M: Clone + Monoid> Add for Bin<A, M> {
    type Output = Bin<A, M>;
    fn add(mut self, mut rhs: Self) -> Self::Output {
        if self.level != rhs.level { return self; }
        let capacity = (1 << (self.level + 1)) - 1;
        let mut measures = Vec::with_capacity(capacity);
        measures.push(self.measure() + rhs.measure());
        self.values.append(&mut rhs.values);
        let mut left = self.measures.into_iter();
        let mut right = rhs.measures.into_iter();
        for i in 0..self.level {
            let pack_size = 1 << i;
            measures.extend(left.by_ref().take(pack_size));
            measures.extend(right.by_ref().take(pack_size));
        }
        Bin { measures, values: self.values, level: self.level + 1 }
    }
}

impl<A: Measured<M>, M: Clone + Monoid> Measured<M> for Bin<A, M> {
    fn measure(&self) -> M {
        self.measures
            .get(0).cloned()
            .or_else(|| self.values.get(0).map(|a| a.measure()))
            .unwrap_or_else(|| M::zero())
    }
}

impl<A, M> Bin<A, M> {
    fn empty() -> Self {
        Bin { measures: Vec::new(), values: Vec::new(), level: 0 }
    }

    fn one(a: A) -> Self {
        Bin { measures: Vec::new(), values: vec![a], level: 0 }
    }
}

impl<A: Measured<M>, M: Clone + Monoid> Bin<A, M> {
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
}

impl<A: Measured<M>, M: Clone + Monoid> RangeCalc for Bin<A, M> {
    type Output = M;

    fn range_calc(&self, from: usize, to: usize) -> Self::Output {
        self.range_iter(0, 0, 1 << self.level, from, to)
    }
}

#[derive(Clone, Debug)]
pub struct Fenwick<A, M> { bins: Vec<Bin<A, M>>, size: usize }

impl<A, M: Monoid> Fenwick<A, M> {
    fn new() -> Self { Fenwick { bins: Vec::new(), size: 0 } }
}

pub trait Monoid: Clone + Add<Self, Output=Self> {
    fn zero() -> Self;
}

impl Monoid for i32 { fn zero() -> Self { 0 } }

impl Monoid for i64 { fn zero() -> Self { 0 } }


pub trait Measured<M: Monoid> {
    fn measure(&self) -> M;
}

impl Measured<i32> for i32 { fn measure(&self) -> Self { *self } }

impl Measured<i64> for i64 { fn measure(&self) -> Self { *self } }


impl<A, M: Monoid> Fenwick<A, M> where A: Measured<M> {
    fn chain_upgrade(&mut self) {
        let mut l = self.bins.len();
        while l > 1 && self.bins[l - 1].level == self.bins[l - 2].level {
            if let (Some(y), Some(x)) = (self.bins.pop(), self.bins.pop()) {
                self.bins.push(x + y)
            }
            l -= 1;
        }
    }
    pub fn push(&mut self, a: A) {
        self.size += 1;
        self.bins.push(Bin::one(a));
        self.chain_upgrade();
    }
}

impl<A: Measured<M>, M: Monoid + Clone> RangeCalc for Fenwick<A, M> {
    type Output = M;

    fn range_calc(&self, mut from: usize, mut to: usize) -> Self::Output {
        let mut m = M::zero();
        for bin in &self.bins {
            m = m + bin.range_calc(from, to);
            let bin_size = 1 << bin.level;
            from = from.max(bin_size) - bin_size;
            to = to.max(bin_size) - bin_size;
            if to == 0 { break; }
        }
        m
    }
}


trait RefUnbox {
    type Output;
    fn ref_unbox(self) -> Self::Output;
}

impl<'a, A> RefUnbox for &'a Option<Box<A>> {
    type Output = Option<&'a A>;

    fn ref_unbox(self) -> Self::Output {
        match self {
            None => None,
            Some(x) => Some(&*x),
        }
    }
}

impl<'a, A, M> IntoIterator for &'a Fenwick<A, M> {
    type Item = &'a A;
    type IntoIter = impl Iterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.bins.iter().flat_map(|b| b.values.iter())
    }
}


impl<A: Measured<M>, M: Monoid + Clone> FromIterator<A> for Fenwick<A, M> {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        let mut res = Fenwick::new();
        for x in iter { res.push(x) }
        res
    }
}

impl<A: Measured<M>, M: Monoid + Clone> Fenwick<A, M> {
    fn calc<R: RangeBounds<I>, I: TryInto<usize> + Copy>(&self, index: R) -> M where {
        let get_bound = |b: Bound<&I>, inc: usize, exc: usize| match b {
            Bound::Unbounded => None,
            Bound::Included(i) => (*i).try_into().ok().map(|x| x + inc),
            Bound::Excluded(i) => (*i).try_into().ok().map(|x| x + exc)
        };
        let left = get_bound(index.start_bound(), 0, 1).unwrap_or(0);
        let right = get_bound(index.end_bound(), 1, 0).unwrap_or(self.size);
        self.range_calc(left, right)
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
    fn test_small() {
        let f: Fenwick<i64, i64> = (0..=7).collect();
        println!("{:?}", f)
    }

    #[test]
    fn test_fenwick() {
        let mut rnd = thread_rng();
        let size: i64 = parse_env("FENWICK_SIZE", 10000000);
        let count: i64 = parse_env("FENWICK_COUNT", 10);
        let f: Fenwick<i64, i64> = (0..=size).collect();
        for _ in 1..count {
            let i = rnd.gen_range(0, size);
            let j = rnd.gen_range(0, size);
            let res = if j <= i { 0 } else { (i + j - 1) * (j - i) / 2 };
            assert_eq!(f.calc(i..j), res);
        }
        println!("{} {}", size, count);
    }
}

