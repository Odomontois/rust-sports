use crate::data::leetcode::{List, ListNode};
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};
use std::iter::{once, repeat};
use crate::data::ignore::Ign;

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        static A: u8 = 'a' as u8;
        let mut qs = [[0; 26]; 2];
        let mut seen = [[false; 26]; 2];
        let mut calc = |xs: Vec<u8>, i: usize| {
            for x in xs {
                qs[i][(x - A) as usize] += 1;
            }
            for j in 0..26 { seen[i][j] = qs[i][j] != 0 }
            qs[i].sort();
        };
        calc(word1.into_bytes(), 0);
        calc(word2.into_bytes(), 1);
        seen[0] == seen[1] && qs[0] == qs[1]
    }
}

pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (n, m) = (mat.len(), mat[0].len());
    let mut d = vec![vec![]; n + m - 1];
    for i in 0..n {
        for j in 0..m {
            d[i + m - 1 - j].push(mat[i][j])
        }
    }
    for v in &mut d {
        v.sort_by_key(|&x| std::cmp::Reverse(x))
    }
    for i in 0..n {
        for j in 0..m {
            if let Some(x) = d[i + m - 1 - j].pop() {
                mat[i][j] = x;
            }
        }
    }
    mat
}


pub fn merge_k_lists1(lists: Vec<List>) -> List {
    let mut heap = BinaryHeap::new();
    let mut res = None;
    let mut prev = &mut res;
    for l in lists.into_iter().flatten() {
        heap.push((Reverse(l.val), Ign(l.next)))
    }
    while let Some((Reverse(val), Ign(next))) = heap.pop() {
        *prev = Some(Box::new(ListNode { val, next: None }));
        if let Some(l) = next {
            heap.push((Reverse(l.val), Ign(l.next)))
        }
        if let Some(b) = prev {
            prev = &mut b.next
        }
    }
    res
}

struct El(Box<ListNode>);

impl PartialEq for El {
    fn eq(&self, other: &Self) -> bool {
        self.0.val == other.0.val
    }
}

impl Eq for El {}

impl PartialOrd for El {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.val.partial_cmp(&other.0.val).map(|o| o.reverse())
    }
}

impl Ord for El {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.val.cmp(&other.0.val).reverse()
    }
}

pub fn merge_k_lists(lists: Vec<List>) -> List {
    let mut res = None;
    let mut prev = &mut res;
    let mut heap: BinaryHeap<_> = lists.into_iter().flatten().map(El).collect();
    while let Some(El(mut b)) = heap.pop() {
        if let Some(l) = b.next {
            heap.push(El(l))
        }
        b.next = None;
        *prev = Some(b);
        prev = &mut prev.as_mut().unwrap().next;
    }
    res
}

pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    nums.into_iter().try_fold(k, |d, x| if x == 1 { if d < k { None } else { Some(0) } } else { Some(d + 1) }).is_some()
}

pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let m = heights[0].len();
    let n = heights.len();
    let mut effs = vec![vec![(None, false); m]; n];
    effs[0][0] = (Some(0), false);
    let mut heap: BinaryHeap<_> = once((Reverse(0), Ign((0, 0)))).collect();
    while let Some((Reverse(eff), Ign((i, j)))) = heap.pop() {
        if effs[i][j].1 { continue; }
        effs[i][j].1 = true;
        if i == n - 1 && j == m - 1 { return eff; }
        for &[u, v] in &[[0, 1], [2, 1], [1, 0], [1, 2]] {
            if i + u == 0 || j + v == 0 || i + u > n || j + v > m { continue; }
            let (x, y) = (i + u - 1, j + v - 1);
            let d = (heights[i][j] - heights[x][y]).abs().max(eff);
            if effs[x][y].0.iter().any(|&x| x <= d) { continue; }
            effs[x][y].0 = Some(d);
            heap.push((Reverse(d), Ign((x, y))));
        }
    }
    panic!("something wrong happened")
}

#[test]
fn check_min_effort() {
    fn check<'a, Arr: 'a>(xs: &'a [Arr], exp: i32) where &'a Arr: IntoIterator<Item=&'a i32> + 'a {
        assert_eq!(
            minimum_effort_path(xs.iter().map(|v| v.into_iter().copied().collect()).collect()),
            exp,
        )
    }
    check(&[[1, 10, 6, 7, 9, 10, 4, 9]], 9);
    check(&[[1, 2, 2], [3, 8, 2], [5, 3, 5]], 2);
    check(&[[1, 2, 3], [3, 8, 4], [5, 3, 5]], 1);
    check(&[[1, 2, 1, 1, 1], [1, 2, 1, 2, 1], [1, 2, 1, 2, 1], [1, 2, 1, 2, 1], [1, 1, 1, 2, 1]], 0);
}

pub fn get_smallest_string(n: i32, k: i32) -> String {
    let d = ((k - n) / 25) as usize;
    let r = ((k - n) % 25) as usize;
    repeat('a')
        .take(n as usize - d - r.min(1))
        .chain(repeat(('a' as u8 + r as u8) as char).take(r.min(1)))
        .chain(repeat('z').take(d))
        .collect()
}





