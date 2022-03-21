use std::{cell::RefCell, collections::HashMap, hash::Hash, iter::once, rc::Rc};

// https://leetcode.com/problems/groups-of-strings/

pub fn group_strings(words: Vec<String>) -> Vec<i32> {
    let bitvec = |s: &str| s.chars().fold(0u32, |acc, c| acc | (1 << (c as u32 - 'a' as u32)));
    let (count, max) = groups(words.iter().map(|s| bitvec(s)));
    vec![count as i32, max as i32]
}

fn groups<A: Hash + Related + Eq + Clone>(words: impl IntoIterator<Item = A>) -> (usize, usize) {
    let mut ws = HashMap::new();
    for w in words {
        *ws.entry(w).or_insert(0usize) += 1
    }
    let ws: HashMap<_, UF<_>> = ws.into_iter().map(|(a, c)| (a.clone(), UF::new(a, c))).collect();
    for (w1, uf1) in &ws {
        for w2 in w1.related() {
            if let Some(uf2) = ws.get(&w2) {
                uf1.union(uf2)
            }
        }
    }
    let root_sizes = || ws.values().filter_map(UF::root_size);
    (root_sizes().count(), root_sizes().max().unwrap_or(0))
}

trait Related {
    type Out: IntoIterator<Item = Self>;
    fn related(&self) -> Self::Out;
}

impl Related for u32 {
    type Out = Box<dyn Iterator<Item = u32>>;
    fn related(&self) -> Self::Out {
        let w = *self;
        let changes = move |i: u32, j: u32| (w & (1 << j) != 0).then(|| w ^ (1 << j) ^ (1 << i));
        let ops = move |i: u32| once(w | 1 << i).chain((i + 1..26).filter_map(move |j| changes(i, j)));
        Box::new((0..26).filter(move |i| w & (1 << i) == 0).flat_map(ops))
    }
}

#[derive(PartialEq, Eq)]
enum UFData<A> {
    Root(usize, A),
    Child(UF<A>),
}

#[derive(Clone, PartialEq, Eq)]
struct UF<A>(Rc<RefCell<UFData<A>>>);

impl<A: Clone + Eq> UF<A> {
    fn new(a: A, size: usize) -> Self {
        UF(RefCell::new(UFData::Root(size, a)).into())
    }

    fn root(&self) -> (Self, usize, A) {
        match &mut *self.0.borrow_mut() {
            UFData::Root(size, a) => (self.clone(), *size, a.clone()),
            UFData::Child(parent) => {
                let (root, size, a) = parent.root();
                *parent = root.clone();
                (root, size, a)
            }
        }
    }

    fn root_size(&self) -> Option<usize> {
        match &*self.0.borrow() {
            UFData::Root(size, _) => Some(*size),
            _ => None,
        }
    }

    fn assign(&self, other: Self) {
        *self.0.borrow_mut() = UFData::Child(other)
    }

    fn set_size(&self, size: usize) {
        if let UFData::Root(my_size, _) = &mut *self.0.borrow_mut() {
            *my_size = size
        }
    }

    fn union(&self, other: &Self) {
        let (self_root, self_size, self_value) = self.root();
        let (other_root, other_size, other_value) = other.root();
        if self_value == other_value {
            return;
        }
        if self_size < other_size {
            self_root.assign(other_root.clone());
            other_root.set_size(self_size + other_size);
        } else {
            other_root.assign(self_root.clone());
            self_root.set_size(self_size + other_size);
        }
    }
}
