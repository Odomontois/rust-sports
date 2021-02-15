use crate::leetcode::data::{Tree, List};
use std::iter::once;
use std::ops::Range;
use std::collections::{HashSet, HashMap, VecDeque};
use std::hash::Hash;

struct Solution();

pub fn is_balanced(root: Tree) -> bool { balanced_depth(root).is_some() }

fn balanced_depth(root: Tree) -> Option<usize> {
    let rc = if let Some(rc) = root { rc } else { return Some(0); };
    let b = rc.borrow();
    let l = balanced_depth(b.left.clone())?;
    let r = balanced_depth(b.right.clone())?;
    if l > r + 1 || r > l + 1 { None } else { Some(l.max(r) + 1) }
}

pub fn next_greater_element(n: i32) -> i32 {
    let s = format!("{}", n);
    let mut digs: Vec<_> = s.chars().rev().collect();
    let mut prev = '0';
    for (i, &c) in digs.iter().enumerate() {
        if c >= prev { prev = c; } else {
            let j = digs.iter().position(|&x| x > c).unwrap();
            digs.swap(i, j);
            // println!("{} {} {:?}", i, j, digs);
            (&mut digs[0..i]).reverse();
            return digs.into_iter().rev().collect::<String>().parse::<i32>().unwrap_or(-1);
        }
    }
    -1
}

#[test]
fn nge() {
    println!("{}", next_greater_element(12));
    println!("{}", next_greater_element(102));
    println!("{}", next_greater_element(201));
    println!("{}", next_greater_element(132000));
    println!("{}", next_greater_element(320001));
    println!("{}", next_greater_element(320010));
    println!("{}", next_greater_element(320100));
    println!("{}", next_greater_element(321000));
}

impl Solution {
    pub fn swap_pairs(head: List) -> List {
        let mut node = *head?;
        let mut next = if let Some(n) = node.next { *n } else {
            return Some(Box::new(node));
        };
        node.next = Self::swap_pairs(next.next);
        next.next = Some(Box::new(node));
        Some(Box::new(next))
    }
}


pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let a = matrix.len();
    let b = matrix.first().map(|v| v.len()).unwrap_or(0);
    if a == 0 || b == 0 { return vec![]; }
    diagonal_coef(a, b).map(|(i, j)| matrix[i][j]).collect()
}


fn box_it<A>(xs: impl IntoIterator<Item=A> + 'static) -> Box<dyn Iterator<Item=A>> { Box::new(xs.into_iter()) }

struct Step<A> { range: Range<A>, rev: bool }

impl<A> Step<A> {
    fn back(mut self) -> Self {
        self.rev = !self.rev;
        self
    }
}

impl<A> Iterator for Step<A> where Range<A>: DoubleEndedIterator<Item=A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rev { self.range.by_ref().rev().next() } else { self.range.next() }
    }
}

fn step<A>(range: Range<A>) -> Step<A> { Step { range, rev: false } }

pub fn diagonal_coef(a: usize, b: usize) -> impl Iterator<Item=(usize, usize)> {
    let n = a.min(b);
    let m = a.max(b);
    let upleft = (1..n).map(|k| (step(0..k).back(), step(0..k)));
    let downright = (0..n - 1).map(move |k| (step(a - k - 1..a).back(), step(b - k - 1..b))).rev();
    let mid = (0..=m - n).map(move |k|
        if a > b { (step(k..n + k).back(), step(0..n)) } else { (step(0..n).back(), step(k..n + k)) }
    );
    let dirs = once(true).chain(once(false)).cycle();
    let it = upleft.chain(mid).chain(downright);
    it.zip(dirs).flat_map(|((is, js), up)|
        if up { is.zip(js) } else { is.back().zip(js.back()) }
    )
}

#[test]
fn diag_check() {
    fn check(a: usize, b: usize) -> Vec<Vec<usize>> {
        let mut x = vec![vec![0; b]; a];
        for (k, (i, j)) in diagonal_coef(a, b).enumerate() {
            if i >= a || j >= b { println!("{} {}", i, j); }

            x[i][j] = k;
        }
        x
    }
    for &(a, b) in &[(4, 4), (5, 5), (5, 3), (3, 5), (1, 1)] {
        for v in &check(a, b) { println!("{:?}", v) }
        println!("-----------")
    }
}

pub fn num_decodings(s: String) -> i32 {
    s.chars().rev().fold((' ', 0, 1), |(p, np, n), c| (c, n, (if ('1'..='9').contains(&c) { n + format!("{}{}", c, p).parse::<i32>().ok().filter(|&i| i <= 26).map(|_| np).unwrap_or(0) } else { 0 }))).2
}

#[test]
fn test_num_decodings() {
    fn check(x: &str, exp: i32) { assert_eq!(num_decodings(x.to_string()), exp) }
    check("12", 2);
    check("226", 3);
    check("0", 0);
    check("1", 1)
}


pub fn min_jumps<A: Copy + Hash + Eq>(arr: Vec<A>) -> i32 {
    let n = arr.len();
    let mut pos = HashMap::<A, Vec<usize>>::new();
    for (i, a) in arr.iter().enumerate() {
        if let Some(v) = pos.get_mut(a) {
            v.push(i);
        } else {
            pos.insert(*a, vec![i]);
        }
    }
    let mut seen: HashSet<_> = once(0).collect();
    let mut seen_val = HashSet::new();
    let mut q: VecDeque<_> = once((0, 0)).collect();
    while let Some((i, steps)) = q.pop_front() {
        let mut add = |z: usize| if seen.insert(z) {
            q.push_back((z, steps + 1));
        };
        if i + 1 == n { return steps; }
        if i > 0 { add(i - 1) }
        if i + 1 < n { add(i + 1) }
        let x = arr[i];
        if seen_val.insert(x) {
            for &i in pos.get(&x).into_iter().flatten() {
                add(i)
            }
        }
    }
    -1
}

#[test]
fn lol() {
    assert_eq!(min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]), 3);
    assert_eq!(min_jumps(vec![7]), 0);
    assert_eq!(min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
    assert_eq!(min_jumps(vec![6, 1, 9]), 2);
    assert_eq!(min_jumps(vec![11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13]), 3);
}

#[test]
fn reach() {
    let mut x = std::collections::BTreeSet::new();
    x.insert(0);
    for i in 1..=20 {
        x = x.into_iter().flat_map(|x| vec![x - i, x + i]).collect();
        let n = i * (i + 1) / 2;
        let exp = if i == 1 { vec![1, -1].into_iter().collect() } else { (n % 2..=n).step_by(2).flat_map(|i| vec![-i, i]).collect() };
        assert_eq!(x, exp);
        // println!("{:?}", x);
    }
}

pub fn reach_number(target: i32) -> i32 {
    let z = target.abs() as u64;
    (0..).scan(0u64, |x, i| {
        *x += i;
        Some(*x)
    }).position(|n| n >= z && n % 2 == z % 2).unwrap_or(0) as i32
}