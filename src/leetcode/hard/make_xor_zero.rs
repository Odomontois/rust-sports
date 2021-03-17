use std::{
    collections::HashMap,
    mem::take,
    ops::{Index, IndexMut},
};

pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
    let mut counters = vec![Counter::new(0); k as usize];
    for (i, &n) in nums.iter().enumerate() {
        counters[i % k as usize][n as u16] += 1;
        counters[i % k as usize].check();
    }
    let mut cur = counters[0].clone();
    for counter in &counters[1..] {
        let mut next = cur.next();
        for (kp, &vp) in &cur {
            for (k, &v) in counter {
                let z = &mut next[k ^ kp];
                *z = (*z).max(vp + v)
            }
        }
        next.check();
        cur = next
    }
    let nonchanging = cur.get(0).unwrap_or(0);
    let mut bests: Vec<_> = counters
        .iter()
        .filter_map(|c| c.into_iter().map(|(_, &v)| v).max())
        .collect();
    bests.sort();
    let changing: u16 = bests[1..].iter().sum();
    nums.len() as i32 - changing.max(nonchanging) as i32
}

const SIZE: usize = 1 << 10;
const THRESHOLD: usize = 1 << 7;

#[derive(Debug, Clone)]
enum Counter {
    Small(HashMap<u16, u16>),
    Big(Vec<u16>),
}

impl Counter {
    pub fn new(cap: usize) -> Self {
        if cap > THRESHOLD {
            Counter::Big(vec![0; SIZE])
        } else {
            Counter::Small(HashMap::new())
        }
    }

    pub fn next(&self) -> Self {
        Self::new(self.len())
    }

    pub fn len(&self) -> usize {
        match self {
            Counter::Small(hm) => hm.len(),
            Counter::Big(_) => SIZE,
        }
    }

    pub fn get(&self, key: u16) -> Option<u16> {
        match self {
            Counter::Small(hm) => hm.get(&key).copied(),
            Counter::Big(v) => Some(v[key as usize]).filter(|&x| x > 0),
        }
    }

    pub fn check(&mut self) {
        match self {
            Counter::Small(hm) if hm.len() > THRESHOLD => {
                let mut elems = vec![0; SIZE];
                for (k, v) in take(hm) {
                    elems[k as usize] = v
                }
                *self = Counter::Big(elems)
            }
            _ => {}
        }
    }
}

impl Index<u16> for Counter {
    type Output = u16;

    fn index(&self, index: u16) -> &Self::Output {
        match self {
            Counter::Small(hm) => &hm[&index],
            Counter::Big(v) => &v[index as usize],
        }
    }
}

impl IndexMut<u16> for Counter {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        match self {
            Counter::Small(hm) => hm.entry(index).or_insert(0),
            Counter::Big(vs) => &mut vs[index as usize],
        }
    }
}

impl<'a> IntoIterator for &'a Counter {
    type Item = (u16, &'a u16);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Counter::Small(hm) => Box::new(hm.iter().map(|(&k, v)| (k, v))),
            Counter::Big(v) => Box::new(v.iter().enumerate().map(|(k, v)| (k as u16, v)).filter(|(_, &v)| v > 0)),
        }
    }
}

#[test]
fn check() {
    assert_eq!(min_changes(vec![1, 2, 0, 3, 0], 3), 3);
    assert_eq!(min_changes(vec![1, 2, 4, 1, 2, 5, 1, 2, 6], 3), 3);
    assert_eq!(min_changes(vec![3, 4, 5, 2, 1, 7, 3, 4, 7], 3), 3);
}

pub fn min_changes_hm(nums: Vec<i32>, k: i32) -> i32 {
    let mut counters = vec![HashMap::new(); k as usize];
    for (i, &n) in nums.iter().enumerate() {
        *counters[i % k as usize].entry(n as u16).or_insert(0) += 1;
    }
    let mut cur = counters[0].clone();
    for counter in &counters[1..] {
        let mut next = HashMap::new();
        for (kp, &vp) in &cur {
            for (k, &v) in counter {
                let z = next.entry(k ^ kp).or_insert(0);
                *z = (*z).max(vp + v)
            }
        }
        cur = next
    }
    let nonchanging = cur.get(&0).copied().unwrap_or(0);
    let mut bests: Vec<_> = counters
        .iter()
        .filter_map(|c| c.into_iter().map(|(_, &v)| v).max())
        .collect();
    bests.sort();
    let changing: u16 = bests[1..].iter().sum();
    nums.len() as i32 - changing.max(nonchanging) as i32
}

#[test]
fn check_hm() {
    assert_eq!(min_changes_hm(vec![1, 2, 0, 3, 0], 3), 3);
    assert_eq!(min_changes_hm(vec![1, 2, 4, 1, 2, 5, 1, 2, 6], 3), 3);
    assert_eq!(min_changes_hm(vec![3, 4, 5, 2, 1, 7, 3, 4, 7], 3), 3);
}


pub fn min_changes_vec(nums: Vec<i32>, k: i32) -> i32 {
    let mut counters = vec![vec![0; SIZE]; k as usize];
    for (i, &n) in nums.iter().enumerate() {
        counters[i % k as usize][n as usize] += 1;
    }
    let mut cur = counters[0].clone();
    for counter in &counters[1..] {
        let mut next = vec![0 ; SIZE];
        for (kp, &vp) in cur.iter().enumerate() {
            for (k, &v) in counter.iter().enumerate() {
                let z = &mut next[k ^ kp];
                *z = (*z).max(vp + v)
            }
        }
        cur = next
    }
    let nonchanging = cur[0];
    let mut bests: Vec<_> = counters
        .iter()
        .filter_map(|c| c.iter().copied().max())
        .collect();
    bests.sort();
    let changing: u16 = bests[1..].iter().sum();
    nums.len() as i32 - changing.max(nonchanging) as i32
}

#[test]
fn check_vec() {
    assert_eq!(min_changes_vec(vec![1, 2, 0, 3, 0], 3), 3);
    assert_eq!(min_changes_vec(vec![1, 2, 4, 1, 2, 5, 1, 2, 6], 3), 3);
    assert_eq!(min_changes_vec(vec![3, 4, 5, 2, 1, 7, 3, 4, 7], 3), 3);
}