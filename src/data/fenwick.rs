use std::iter::{empty, FromIterator, once};
use std::mem::swap;
use std::ops::{Add, Bound, RangeBounds};
use std::convert::TryInto;


#[derive(Clone, Debug)]
pub enum FullBin<A, M> {
    Empty,
    Leaf(A),
    Node(Box<FullBin<A, M>>, Box<FullBin<A, M>>, M),
}

impl<A: Measured<M>, M: Clone + Monoid> Add for FullBin<A, M> {
    type Output = FullBin<A, M>;
    fn add(self, rhs: Self) -> Self::Output {
        let m = self.measure() + rhs.measure();
        FullBin::Node(Box::new(self), Box::new(rhs), m)
    }
}

impl<A, M: Clone + Monoid> FullBin<A, M> where A: Measured<M> {
    fn measure(&self) -> M {
        match self {
            FullBin::Empty => M::zero(),
            FullBin::Leaf(a) => a.measure(),
            FullBin::Node(_, _, m) => m.clone()
        }
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=&'a A> + 'a> {
        match self {
            FullBin::Empty => Box::new(empty()),
            FullBin::Leaf(a) => Box::new(once(a)),
            FullBin::Node(l, r, _) => Box::new(l.iter().chain(r.iter()))
        }
    }

    fn range_calc(&self, from: usize, until: usize, size: usize) -> M {
        if from >= size || from >= until { return M::zero(); }
        if from == 0 && until >= size { return self.measure(); }
        let s2 = size / 2;
        match self {
            FullBin::Empty => M::zero(),
            FullBin::Leaf(a) => a.measure(),
            FullBin::Node(l, r, _) =>
                if from >= s2 {
                    r.range_calc(from - s2, until - s2, s2)
                } else if until <= s2 {
                    l.range_calc(from, until, s2)
                } else {
                    l.range_calc(from, s2, s2) + r.range_calc(0, until - s2, s2)
                },
        }
    }
}

#[derive(Clone, Debug)]
pub struct HalfBin<A, M> {
    left: FullBin<A, M>,
    right: Option<Box<HalfBin<A, M>>>,
    measure: M,
    level: usize,
}


impl<A, M: Clone + Monoid> HalfBin<A, M> where A: Measured<M> {
    fn one(a: A) -> HalfBin<A, M> {
        let measure = a.measure();
        HalfBin {
            left: FullBin::Leaf(a),
            right: None,
            measure,
            level: 1,
        }
    }

    fn left_concat(&mut self, b: FullBin<A, M>) {
        let mut sw = FullBin::Empty;
        swap(&mut self.left, &mut sw);
        self.left = sw + b;
    }

    fn upgrade(&mut self) {
        let mut rsw = None;
        swap(&mut self.right, &mut rsw);
        if let Some(hb) = rsw {
            self.left_concat(hb.left);
            self.right = hb.right;
        }
        self.level += 1;
    }

    fn push(&mut self, a: A) -> bool {
        self.measure = self.measure.clone() + a.measure();

        if self.level == 0 {
            self.left = FullBin::Leaf(a);
            self.level = 1;
            return true;
        }

        if let Some(rb) = &mut self.right {
            let upgraded = rb.push(a);
            if upgraded && rb.level == self.level {
                self.upgrade();
                true
            } else {
                false
            }
        } else {
            self.right = Some(Box::new(Self::one(a)));
            if self.level == 1 {
                self.upgrade();
                true
            } else { false }
        }
    }

    fn range_calc(&self, from: usize, until: usize) -> M {
        if self.level == 0 || until <= from { return <M>::zero(); }
        let bin_size = 2usize.pow(self.level as u32 - 1);
        if from >= bin_size {
            return if let Some(b) = &self.right {
                b.range_calc(from - bin_size, until - bin_size)
            } else { M::zero() };
        }
        let lm = self.left.range_calc(from, until, bin_size);
        if until <= bin_size { lm } else if let Some(rb) = &self.right {
            lm + rb.range_calc(0, until - bin_size)
        } else { lm }
    }
}

#[derive(Clone, Debug)]
pub struct Fenwick<A, M> { bin: HalfBin<A, M>, size: usize }

impl<A, M: Monoid> Fenwick<A, M> {
    fn new() -> Self {
        Fenwick { bin: HalfBin { left: FullBin::Empty, right: None, measure: <M>::zero(), level: 0 }, size: 0 }
    }
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
    pub fn push(&mut self, a: A) {
        self.bin.push(a);
        self.size += 1;
    }
}

pub struct FenwickIterator<'a, A, M> { stack: Vec<&'a FullBin<A, M>>, tail: Option<&'a HalfBin<A, M>> }

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

impl<'a, A, M> Iterator for FenwickIterator<'a, A, M> {
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(head) = self.stack.pop() {
            match head {
                FullBin::Leaf(a) => return Some(a),
                FullBin::Node(l, r, _) => {
                    self.stack.push(r);
                    self.stack.push(l);
                }
                FullBin::Empty => {}
            }
        }
        let mut swt = None;
        swap(&mut swt, &mut self.tail);
        let HalfBin { left, right, .. } = swt?;
        self.stack.push(left);
        self.tail = right.ref_unbox();
        self.next()
    }
}

impl<'a, A, M> IntoIterator for &'a HalfBin<A, M> {
    type Item = &'a A;
    type IntoIter = FenwickIterator<'a, A, M>;

    fn into_iter(self) -> Self::IntoIter {
        FenwickIterator { stack: vec![&self.left], tail: (&self.right).ref_unbox() }
    }
}

impl<'a, A, M> IntoIterator for &'a Fenwick<A, M> {
    type Item = &'a A;
    type IntoIter = FenwickIterator<'a, A, M>;

    fn into_iter(self) -> Self::IntoIter { (&self.bin).into_iter() }
}

impl<'a, A, M> IntoIterator for &'a FullBin<A, M> {
    type Item = &'a A;
    type IntoIter = FenwickIterator<'a, A, M>;

    fn into_iter(self) -> Self::IntoIter {
        FenwickIterator { stack: vec![&self], tail: None }
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
        self.bin.range_calc(left, right)
    }
}


#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};
    use std::env;
    use std::str::FromStr;
    use super::*;


    fn parse_env<A: FromStr>(name: &str, default: A) -> A {
        env::var(name).ok().and_then(|s| s.parse().ok()).unwrap_or(default)
    }

    #[test]
    fn test_fenwick() {
        let mut rnd = thread_rng();
        let size: i64 = parse_env("FENWICK_SIZE", 1000000);
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

