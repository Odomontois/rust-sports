use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
    ops::{AddAssign, SubAssign},
};

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
    let mut nums_fwd = nums.iter().map(|&x| x);
    let nums_bck = nums_fwd.clone();
    let calc = |&Item { count, value }: &Item<i32>| count as i64 * value as i64;
    let mut xsum = XSum::new(x as usize, calc);
    for num in nums_fwd.by_ref().take(k as usize - 1) {
        xsum.change(num, 1);
    }
    nums_fwd
        .zip(nums_bck)
        .map(|(add, remove)| {
            xsum.change(add, 1);
            let res = xsum.data.top_sum;
            xsum.change(remove, -1);
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
}

struct XSum<A, S, F> {
    data: XSumData<A, S>,
    calc: F,
    top_len: usize,
}

impl<A: Default, S: Default, F: Fn(&Item<A>) -> S> XSum<A, S, F> {
    fn new(top_len: usize, calc: F) -> Self {
        Self { data: <_>::default(), top_len, calc }
    }
}

impl<A: Ord + Copy + Hash + Eq, S: AddAssign + SubAssign, F: Fn(&Item<A>) -> S> XSum<A, S, F> {
    pub fn change(&mut self, value: A, dx: i32) {
        let Self { data, calc, .. } = self;
        let freq = data.frequency.entry(value).or_insert(0);
        let prev = Item { count: *freq, value };
        if data.top.remove(&prev) {
            data.top_sum -= calc(&prev);
        } else {
            data.bottom.remove(&prev);
        }
        *freq += dx;
        let new = Item { count: *freq, value };
        if data.top.first().filter(|&top| top <= &new).is_some() {
            data.top_sum += calc(&new);
            data.top.insert(new);
        } else {
            data.bottom.insert(new);
        }
        if data.top.len() > self.top_len {
            if let Some(item) = data.top.pop_first() {
                data.top_sum -= calc(&item);
                data.bottom.insert(item);
            }
        } else if data.top.len() < self.top_len {
            if let Some(item) = data.bottom.pop_last() {
                data.top_sum += calc(&item);
                data.top.insert(item);
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
