use std::collections::HashMap;
use crate::data::leetcode::{List, ListNode};

pub fn can_form_array(arr: Vec<i32>, mut pieces: Vec<Vec<i32>>) -> bool {
    let mut rev: Vec<Option<usize>> = vec![None; 101];
    for (i, &a) in arr.iter().enumerate() {
        rev[a as usize] = Some(i);
    }
    pieces.sort_by_key(|v| rev[v[0] as usize]);
    pieces.concat() == arr
}


struct Solution;

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        Arrangement::new(n).count(0, 0) as i32
    }

    // fn count(&mut cache: HashMap<u64, u64>, possibilities: &Vec<Vec<i32>>) -> u64 {
    //     if i == possibilities.len() { return 1; }
    //     *self.cache.entry(mark).or_insert_with(||
    //         self.possibilities[i].clone().into_iter()
    //             .filter(|j| mark & (1 << *j) == 0)
    //             .map(|j| self.count(mark | (1 << j), i + 1))
    //             .sum())
    // }
}

struct Arrangement { possibilities: Vec<Vec<i32>>, cache: HashMap<u64, u64> }

impl Arrangement {
    fn new(n: i32) -> Self {
        let mut possibilities: Vec<Vec<_>> = (1..=n).map(|i|
            (1..=n).filter(|j| i % j == 0 || j % i == 0).collect()
        ).collect();
        possibilities.sort_by_key(<Vec<_>>::len);
        Arrangement { possibilities, cache: HashMap::new() }
    }

    fn count(&mut self, mark: u64, i: usize) -> u64 {
        if i == self.possibilities.len() { return 1; }
        if let Some(&res) = self.cache.get(&mark) { return res; }
        let res = self.possibilities[i].clone().into_iter()
            .filter(|j| mark & (1 << *j) == 0)
            .map(|j| self.count(mark | (1 << j), i + 1))
            .sum();
        self.cache.insert(mark, res);
        res
    }
}

#[test]
fn arrangement() {
    let count: i32 = std::env::var("SIZE").ok().and_then(|x| x.parse().ok()).unwrap_or(15);
    println!("{}", Arrangement::new(count).count(0, 0));
}

impl Solution {
    pub fn merge_two_lists(l1: List, l2: List) -> List {
        let n1 = if let Some(n) = &l1 { &**n } else { return l2; };
        let n2 = if let Some(n) = &l2 { &**n } else { return l1; };
        if n1.val <= n2.val {
            l1.map(|b| Box::new(ListNode { val: b.val, next: Self::merge_two_lists(b.next, l2) }))
        } else {
            l2.map(|b| Box::new(ListNode { val: b.val, next: Self::merge_two_lists(l1, b.next) }))
        }
    }
}