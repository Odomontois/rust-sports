use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    mem::swap,
};

use crate::data::leetcode::Tree;

struct Search<'a> {
    nums: &'a [i32],
    from: i32,
    until: i32,
    sum: i32,
}
impl<'a> Clone for Search<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Copy for Search<'a> {}

impl<'a> Search<'a> {
    fn make(nums: &'a [i32], from: i32, until: i32) -> Self {
        let sum = nums.iter().filter(|&x| (from..until).contains(x)).sum();
        Self { nums, from, until, sum }
    }

    fn expected(self) -> i32 {
        (self.until + self.from - 1) * (self.until - self.from) / 2
    }
    fn normal(self) -> bool {
        self.sum == self.expected()
    }
    fn diff(self) -> i32 {
        self.expected() - self.sum
    }
    fn missing(self) -> Option<i32> {
        Some(self.diff()).filter(|&x| x > 0)
    }
    fn duplicate(self) -> Option<i32> {
        Some(-self.diff()).filter(|&x| x > 0)
    }
    fn split(self) -> (Self, Self) {
        let m = (self.from + self.until) / 2;
        (
            Self::make(self.nums, self.from, m),
            Self::make(self.nums, m, self.until),
        )
    }
}

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut search = Search::make(&nums, 1, nums.len() as i32 + 1);
    loop {
        let (l, r) = search.split();
        if l.normal() {
            search = r
        } else if r.normal() {
            search = l
        } else {
            return vec![
                l.duplicate().or(r.duplicate()).unwrap(),
                l.missing().or(r.missing()).unwrap(),
            ];
        }
    }
}

#[test]
fn check_find_error_nums() {
    assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3])
}

pub fn average_of_levels(root: Tree) -> Vec<f64> {
    let mut v = vec![];
    fill_average(&mut v, 0, root);
    v.into_iter().map(|(s, c)| s as f64 / c as f64).collect()
}

fn fill_average(v: &mut Vec<(i64, usize)>, i: usize, root: Tree) -> Option<()> {
    let r = root?;
    let node = r.borrow();
    if i >= v.len() {
        v.push((0, 0))
    }
    let k = &mut v[i];
    k.0 += node.val as i64;
    k.1 += 1;
    fill_average(v, i + 1, node.left.clone());
    fill_average(v, i + 1, node.right.clone());
    Some(())
}

struct CopyHashMap<K, V> {
    size: usize,
    items: Vec<Vec<(K, V)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl<K: Clone + Hash + Eq, V: Clone> CopyHashMap<K, V> {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    fn with_capacity(n: usize) -> Self {
        Self {
            size: 0,
            items: vec![vec![]; n],
        }
    }

    fn resize(&mut self) {
        let mut old = Self::with_capacity(self.size * 4 + 1);
        swap(self, &mut old);
        for (k, v) in old.items.into_iter().flatten() {
            self.put(k, v)
        }
    }

    fn bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.items.len()
    }

    fn search_key(&mut self, key: &K) -> (&mut Vec<(K, V)>, Option<usize>) {
        let bucket = self.bucket(&key);
        let lst = &mut self.items[bucket];
        let pos = lst.iter().position(|(k, _)| k == key);
        (lst, pos)
    }

    /** value will always be non-negative. */
    pub fn put(&mut self, key: K, value: V) {
        if self.items.len() < self.size * 2 + 1 {
            self.resize()
        }
        let (lst, pos) = self.search_key(&key);
        if let Some(pos) = pos {
            lst[pos].1 = value
        } else {
            lst.push((key, value));
            self.size += 1;
        }
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get_opt(&self, key: &K) -> Option<V> {
        for (k, v) in &self.items[self.bucket(key)] {
            if k == key {
                return Some(v.clone());
            }
        }
        None
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: K) {
        let (lst, pos) = self.search_key(&key);
        if let Some(i) = pos {
            lst.remove(i);
            self.size -= 1;
        }
        if self.size * 8 + 1 < self.items.len() {
            self.resize()
        }
    }
}
type MyHashMap = CopyHashMap<i32, i32>;

impl<K: Clone + Hash + Eq> CopyHashMap<K, i32> {
    fn get(&self, key: K) -> i32 {
        self.get_opt(&key).unwrap_or(-1)
    }
}

#[test]
fn check1() {
    let mut hm = MyHashMap::new();
    hm.remove(100);
    hm.put(1, 1);
    hm.put(2, 2);
    assert_eq!(hm.get(1), 1);
    assert_eq!(hm.get(3), -1);
    hm.put(2, 1);
    assert_eq!(hm.get(2), 1);
    hm.remove(2);
    assert_eq!(hm.get(2), -1);
}

