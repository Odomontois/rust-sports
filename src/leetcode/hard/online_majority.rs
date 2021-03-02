use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Range,
};

use crate::data::segpos::SegPos;

type ItemVec<A> = Vec<(A, usize)>;
type ItemRange<A> = (Option<A>, HashMap<A, usize>);
struct Majority<A> {
    elems: Vec<A>,
    counts: Vec<ItemRange<A>>,
}

enum Quant<'a, A> {
    Single(&'a A),
    Many(Option<&'a A>, &'a HashMap<A, usize>),
}

impl<'a, A> Clone for Quant<'a, A> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, A> Copy for Quant<'a, A> {}

impl<'a, A> Quant<'a, A> {
    fn of(items: &'a ItemRange<A>) -> Self {
        Quant::Many(items.0.as_ref(), &items.1)
    }

    fn len(self) -> usize {
        match self {
            Quant::Single(_) => 1,
            Quant::Many(_, m) => m.len(),
        }
    }

    fn popular(self) -> Option<&'a A> {
        match self {
            Quant::Single(a) => Some(a),
            Quant::Many(ao, _) => ao,
        }
    }
}

impl<'a, A: Hash + Eq> Quant<'a, A> {
    fn quantity(self, a: &A) -> usize {
        match self {
            Quant::Single(b) => (a == b) as usize,
            Quant::Many(_, m) => m.get(a).copied().unwrap_or(0),
        }
    }
}

impl<A: Clone + Ord + Hash> Majority<A> {
    fn start(&self) -> SegPos {
        SegPos ::start(self.elems.len())
    }

    fn zip<X, Y>(ox: Option<X>, oy: Option<Y>) -> Option<(X, Y)> {
        match (ox, oy) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    }

    fn merge(v1: ItemVec<A>, v2: ItemVec<A>) -> ItemVec<A> {
        let mut res = Vec::new();
        let mut it1 = v1.into_iter().peekable();
        let mut it2 = v2.into_iter().peekable();
        while let Some(((x, _), (y, _))) = Self::zip(it1.peek(), it2.peek()) {
            res.push(if x == y {
                let (x, cx) = it1.next().unwrap();
                let (_, cy) = it2.next().unwrap();
                (x, cx + cy)
            } else if x > y {
                it1.next().unwrap()
            } else {
                it2.next().unwrap()
            })
        }
        res.extend(it1);
        res.extend(it2);

        res
    }

    fn init_fill(&mut self, pos: SegPos) -> ItemVec<A> {
        if pos.elem() {
            let x = self.elems[pos.from].clone();
            return vec![(x, 1)];
        }
        let p = pos.p;
        let (l, r) = pos.subs();
        let lm = self.init_fill(l);
        let rm = self.init_fill(r);
        let res = Self::merge(lm, rm);
        let m = res.iter().max_by_key(|(_, c)| *c).unwrap().0.clone();
        self.counts[p] = (Some(m), res.iter().cloned().collect());
        res
    }

    fn new(arr: Vec<A>) -> Self {
        let init = (None, HashMap::new());
        let mut check = Self {
            counts: vec![init; arr.len()],
            elems: arr,
        };
        check.init_fill(check.start());

        check
    }

    fn quantities<'a>(&'a self, pos: SegPos, range: &Range<usize>, v: &mut Vec<Quant<'a, A>>) {
        if !pos.intersects(range) {
            return;
        }
        if pos.elem() {
            v.push(Quant::Single(&self.elems[pos.from]));
            return;
        }
        if pos.inside(range) {
            v.push(Quant::of(&self.counts[pos.p]));
            return;
        }
        let (l, r) = pos.subs();
        self.quantities(l, range, v);
        self.quantities(r, range, v);
    }

    fn run_query(&self, left: usize, right: usize, threshold: usize) -> Option<&A> {
        let mut qs = vec![];
        self.quantities(self.start(), &(left..right + 1), &mut qs);
        qs.sort_by_key(|q| Reverse(q.len()));
        let mut seen: HashSet<A> = HashSet::new();
        let mut remain = right - left + 1;
        let quantity = |el: &A| -> usize { qs.iter().map(|q| q.quantity(el)).sum() };
        for el in qs.iter().flat_map(|q| q.popular()) {
            if !seen.insert(el.clone()) {
                continue;
            }
            let q = quantity(el);
            if q >= threshold {
                return Some(el);
            }
            remain -= q;
            if remain < threshold {
                break;
            }
        }

        None
    }
}

type MajorityChecker = Majority<i32>;

impl MajorityChecker {
    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        self.run_query(left as usize, right as usize, threshold as usize)
            .copied()
            .unwrap_or(-1)
    }
}

#[test]
fn check_majority() {
    fn check(vals: &[i32], args: &[[i32; 3]], exp: &[i32]) {
        let check = MajorityChecker::new(vals.to_vec());
        for (&[l, r, t], &exp) in args.iter().zip(exp) {
            assert_eq!(check.query(l, r, t), exp, "l = {} r = {} t = {}", l, r, t);
        }
    }

    check(&[1, 1, 2, 2, 1, 1], &[[0, 5, 4], [0, 3, 3], [2, 3, 2]], &[1, -1, 2]);
    check(
        &[1, 1, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 1, 1],
        &[
            [0, 10, 9],
            [0, 10, 7],
            [0, 10, 6],
            [5, 7, 3],
            [2, 3, 2],
            [2, 9, 7],
            [2, 9, 5],
            [0, 1, 2],
            [0, 2, 2],
            [12, 14, 2],
            [13, 14, 2],
        ],
        &[1, 3, 3, 3, -1, 3, 3, 1, 1, 1, 1],
    );
}
