use std::collections::{HashSet, BTreeMap};
use std::iter::{FromIterator, once};

use crate::data::leetcode::Tree;

#[allow(dead_code)]
pub fn rob(root: Tree) -> i32 {
    rob_iter(&root).0
}

fn rob_iter(tree: &Tree) -> (i32, i32) {
    if let Some(noderef) = tree {
        let r = noderef.borrow();
        let (lall, ldown) = rob_iter(&r.left);
        let (rall, rdown) = rob_iter(&r.right);
        let down = lall + rall;
        let all = (r.val + ldown + rdown).max(down);
        (all, down)
    } else {
        (0, 0)
    }
}


#[test]
fn test_calculate() {
    fn check(s: &str, res: i32) { assert_eq!(calculate(s.to_string()), res) }
    check("3+2*2", 7);
    check("  3/2   ", 1);
    check("3+5    /   2", 5);
    println!("{}", std::mem::size_of::<Calculate>());
    println!("{}", std::mem::size_of::<(Option<AddOp>, Option<MulOp>)>());
}

#[allow(dead_code)]
pub fn calculate(s: String) -> i32 {
    s.chars().collect::<Calculate>().result()
}

enum AddOp { Plus, Minus }

enum MulOp { Times, Div }

struct Calculate { add: i32, add_op: Option<AddOp>, mul: i32, mul_op: Option<MulOp>, cur: i32 }

impl Calculate {
    fn result(self) -> i32 { self.next_add() }

    fn next_add(&self) -> i32 {
        let m = self.next_mul();
        match self.add_op {
            Some(AddOp::Plus) => self.add + m,
            Some(AddOp::Minus) => self.add - m,
            None => m
        }
    }
    fn next_mul(&self) -> i32 {
        match self.mul_op {
            Some(MulOp::Times) => self.mul * self.cur,
            Some(MulOp::Div) => self.mul / self.cur,
            None => self.cur
        }
    }
    fn feed_mul(&mut self, op: MulOp) {
        self.mul = self.next_mul();
        self.mul_op = Some(op);
        self.cur = 0
    }
    fn feed_add(&mut self, op: AddOp) {
        self.add = self.next_add();
        self.add_op = Some(op);
        self.mul_op = None;
        self.cur = 0;
    }
    fn feed(&mut self, c: char) {
        match (c, c.to_digit(10)) {
            (_, Some(d)) => self.cur = self.cur * 10 + d as i32,
            ('*', _) => self.feed_mul(MulOp::Times),
            ('/', _) => self.feed_mul(MulOp::Div),
            ('+', _) => self.feed_add(AddOp::Plus),
            ('-', _) => self.feed_add(AddOp::Minus),
            _ => {}
        }
    }
}

impl FromIterator<char> for Calculate {
    fn from_iter<T: IntoIterator<Item=char>>(iter: T) -> Self {
        let mut calc = Calculate { add: 0, add_op: None, mul: 1, mul_op: None, cur: 0 };
        for c in iter { calc.feed(c) }
        calc
    }
}

#[allow(dead_code)]
pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    let mut cur = 1 % k;
    let mut n = 1;
    let mut seen = HashSet::new();
    while cur != 0 {
        seen.insert(cur);
        n += 1;
        cur = (cur * 10 + 1) % k;
        if seen.contains(&cur) { return -1; }
    }
    n
}


pub struct SolLongest;


impl SolLongest {
    #[allow(dead_code)]
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let mut counts = [0; 26];
        for c in s.chars() {
            let u = c.min('z').max('a') as usize - 'a' as usize;
            counts[u] += 1;
        }
        let min_ch = counts.iter().enumerate().filter(|&(_, &u)| u > 0 && u < k).min_by_key(|&(_, &u)| u);
        if let Some((ci, _)) = min_ch {
            let c = (ci as u8 + 'a' as u8) as char;
            s.split(c).filter(|s| !s.is_empty()).map(|sub| Self::longest_substring(sub.to_string(), k)).max().unwrap_or(0)
        } else { s.len() as i32 }
    }
}

#[test]
fn test_longest_substring() {
    fn check(s: &str, i: i32, exp: i32) { assert_eq!(SolLongest::longest_substring(s.to_string(), i), exp) }
    check("aaabb", 3, 3);
}


#[allow(dead_code)]
pub fn can_partition(nums: Vec<i32>) -> bool {
    let n: i32 = nums.iter().cloned().sum();
    if n % 2 == 1 { return false; }
    let mut seen = [false; 30000];
    let mut q = vec![0];
    for m in nums {
        for i in q.iter().copied().map(|i| i + m as usize).filter(|&i| !seen[i]).collect::<Vec<_>>() {
            seen[i] = true;
            q.push(i)
        }
    }
    seen[n as usize / 2]
}

#[test]
fn can_partition_test() {
    assert_eq!(can_partition(vec![3, 3, 3, 4, 5]), true)
}

#[allow(dead_code)]
pub fn can_partition_1(nums: Vec<i32>) -> bool {
    let n: i32 = nums.iter().cloned().sum();
    n % 2 == 0 && nums.into_iter().fold(once(0).collect::<HashSet<_>>(), |s, n| s.into_iter().flat_map(|m| vec![m, m + n]).collect()).contains(&(n / 2))
}


#[allow(dead_code)]
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    type Counter = BTreeMap<i32, i32>;
    let (start, cont) = nums.split_at(k as usize - 1);
    let mut window = BTreeMap::<i32, i32>::new();
    let mut res = vec![];
    fn add(w: &mut Counter, k: i32, d: i32) {
        let cnt = w.remove(&k).unwrap_or(0) + d;
        if cnt > 0 { w.insert(k, cnt); }
    }

    for &k in start { add(&mut window, k, 1); }
    for (&left, &right) in nums.iter().zip(cont.iter()) {
        add(&mut window, right, 1);
        if let Some(&x) = window.keys().rev().next() { res.push(x) }
        add(&mut window, left, -1);
    }
    res
}

#[test]
fn test_window() {
    println!("{:?}", max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3));
    println!("{:?}", max_sliding_window(vec![1, -1], 1));
}