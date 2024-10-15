use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
    ops::{AddAssign, SubAssign},
};

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
    let mut nums_fwd = nums.iter().map(|&x| x as i64);
    let nums_bck = nums_fwd.clone();
    let calc = |Item { count, value }| count as i64 * value as i64;
    let mut xsum = XSumData::new(x as usize);
    for num in nums_fwd.by_ref().take(k as usize - 1) {
        xsum.change(num, 1, calc);
    }
    nums_fwd
        .zip(nums_bck)
        .map(|(add, remove)| {
            xsum.change(add, 1, calc);
            let res = xsum.top_sum;
            xsum.change(remove, -1, calc);
            res
        })
        .collect()
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug)]
struct Item<A> {
    count: i32,
    value: A,
}
#[derive(Debug, Default)]
struct XSumData<A, S> {
    top: BTreeSet<Item<A>>,
    bottom: BTreeSet<Item<A>>,
    frequency: HashMap<A, i32>,
    top_sum: S,
    top_len: usize,
}

struct XSum<A, S, F> {
    data: XSumData<A, S>,
    calc: F,
}

impl<A: Default, S: Default, F> XSumD<A, S> {
    fn new(top_len: usize) -> Self {
        Self { top_len, ..Self::default() }
    }
}

impl<A: Ord + Copy + Hash + Eq, S: AddAssign + SubAssign> XSumData<A, S> {
    fn change(&mut self, value: A, dx: i32, calc: impl Fn(Item<A>) -> S) {
        let freq = self.frequency.entry(value).or_insert(0);
        let prev = Item { count: *freq, value };
        if self.top.remove(&prev) {
            self.top_sum -= calc(prev);
        } else {
            self.bottom.remove(&prev);
        }
        *freq += dx;
        let new = Item { count: *freq, value };
        if self.top.first().filter(|&top| top <= &new).is_some() {
            self.top_sum += calc(new);
            self.top.insert(new);
        } else {
            self.bottom.insert(new);
        }
        if self.top.len() > self.top_len {
            if let Some(item) = self.top.pop_first() {
                self.top_sum -= calc(item);
                self.bottom.insert(item);
            }
        } else if self.top.len() < self.top_len {
            if let Some(item) = self.bottom.pop_last() {
                self.top_sum += calc(item);
                self.top.insert(item);
            }
        }
    }
}

#[test]
fn example1() {
    assert_eq!(find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2), vec![6, 10, 12]);
}

#[test]
fn example2() {
    assert_eq!(find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2), vec![11, 15, 15, 15, 12]);
}

#[test]
fn wa1() {
    assert_eq!(find_x_sum(vec![10, 7, 6, 9, 8], 2, 1), vec![10, 7, 9, 9]);
}
