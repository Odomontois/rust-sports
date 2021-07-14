use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::once;

use crate::data::leetcode::{tree, Tree};

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Tree {
    let n = inorder.len();
    Build::new(preorder, inorder).calc(0, 0, n)
}

struct Build {
    preorder: Vec<i32>,
    inorder: Vec<i32>,
    index: HashMap<i32, usize>,
}

impl Build {
    fn new(preorder: Vec<i32>, inorder: Vec<i32>) -> Self {
        Self {
            preorder,
            index: inorder.iter().enumerate().map(|(i, &x)| (x, i)).collect(),
            inorder,
        }
    }

    fn calc(&self, pix: usize, iix: usize, eix: usize) -> Tree {
        if iix == eix {
            return None;
        }
        let root = self.preorder[pix];
        let rix = self.index[&root];
        tree(
            root,
            self.calc(pix + 1, iix, rix),
            self.calc(pix + 1 + rix - iix, rix + 1, eix),
        )
    }
}

pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut best = vec![0; nums.len()];
    let mut tail = BTreeSet::<(i32, usize)>::new();
    for (i, &num) in nums.iter().enumerate() {
        let cur = num + tail.iter().rev().next().map(|&(x, _)| x).unwrap_or(0);
        best[i] = cur;
        tail.insert((cur, i));
        if i >= k {
            tail.remove(&(best[i - k], i - k));
        }
    }
    best[best.len() - 1]
}

#[derive(Default)]
struct MyCalendar(BTreeMap<i32, i32>);

impl MyCalendar {
    fn new() -> Self {
        Self(once((std::i32::MIN, std::i32::MAX)).collect())
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let (&from, &to) = self.0.range(..=start).rev().next().unwrap();
        if to < end {
            return false;
        }
        self.0.insert(from, start);
        self.0.insert(end, to);
        true
    }
}

pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
    let n = stones.len();
    let sums: Vec<_> = once(0)
        .chain(stones.into_iter().scan(0, |s, x| {
            *s += x;
            Some(*s)
        }))
        .collect();
    (1..n).fold(vec![0; n], |p, i| {
        (1..p.len())
            .map(|j| (sums[j + i] - sums[j] - p[j]).max(sums[j + i - 1] - sums[j - 1] - p[j - 1]))
            .collect()
    })[0]
}

#[test]
fn stone_game_test() {
    fn check(elems: &[i32], exp: i32) {
        assert_eq!(stone_game_vii(elems.to_vec()), exp)
    }
    check(&[5, 3, 1, 4, 2], 6);
    check(&[7, 90, 5, 1, 100, 10, 10, 2], 122);
}

// use std::iter::once;

pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
    stations
        .into_iter()
        .map(|x| [x[0], x[1]])
        .chain(once([target, 0]))
        .fold((0, vec![start_fuel]), |(p, xs), [c, f]| {
            let prev = || xs.iter().map(|x| x + p - c);
            let new = prev().map(|g| if g < 0 { g } else { g + f });
            let next = prev()
                .chain(once(-1))
                .zip(once(-1).chain(new))
                .map(|(x, y)| x.max(y))
                .collect();
            (c, next)
        })
        .1
        .into_iter()
        .enumerate()
        .filter(|&(_, v)| v >= 0)
        .map(|(i, _)| i as i32)
        .min()
        .unwrap_or(-1)
}

#[test]
fn min_refuel_test() {
    fn check(exp: i32, target: i32, start: i32, stops: &[[i32; 2]]) {
        assert_eq!(
            exp,
            min_refuel_stops(target, start, stops.into_iter().map(|x| x.to_vec()).collect())
        )
    }
    check(0, 1, 1, &[]);
    check(-1, 100, 1, &[[10, 100]]);
    check(2, 100, 10, &[[10, 60], [20, 30], [30, 30], [60, 40]]);
}

pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
    let wi: Vec<_> = words.iter().enumerate().map(|(i, w)| (w as &str, i)).collect();
    let ex: HashMap<_, _> = wi.iter().copied().collect();

    let mut res = HashSet::new();
    for (w, i) in wi {
        for l in 0..w.len() {
            let s: String = w[..l].chars().rev().collect();
            if let Some(&j) = ex.get(s.as_str()) {
                if i != j && w.chars().chain(s.chars()).eq(s.chars().rev().chain(w.chars().rev())) {
                    res.insert(vec![i as i32, j as i32]);
                }
            }
            let s: String = w[l..].chars().rev().collect();
            if let Some(&j) = ex.get(s.as_str()) {
                if i != j && s.chars().chain(w.chars()).eq(w.chars().rev().chain(s.chars().rev())) {
                    res.insert(vec![j as i32, i as i32]);
                }
            }
        }
    }
    res.into_iter().collect()
}
