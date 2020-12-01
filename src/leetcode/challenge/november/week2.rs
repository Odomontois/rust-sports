use crate::leetcode::data::Tree;

#[allow(dead_code)]
fn sprod(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> i32 {
    (p[0] - q[0]) * (p[0] - r[0]) + (p[1] - q[1]) * (p[1] - r[1])
}

#[allow(dead_code)]
fn square(ps: Vec<&Vec<i32>>) -> bool {
    let sides: Vec<_> = ps.windows(2).map(|xs| sprod(xs[0], xs[1], xs[1])).collect();
    sides[0] > 0 && sides.windows(2).all(|x| x[0] == x[1]) && sprod(ps[0], ps[1], ps[3]) == 0
}

#[allow(dead_code)]
pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    let points = [&p1, &p2, &p3, &p4];
    (1..4usize).flat_map(
        |i| (1..4).filter_map(move |j| if j == i { None } else { Some(vec![0, i, j, 6 - i - j, 0]) })
    ).any(|perm| square(perm.iter().map(|&i| points[i]).collect()))
}


#[allow(dead_code)]
fn tilt_sum(root: Tree) -> (i32, i32) {
    if let Some(r) = root {
        let node = r.borrow();
        let (lsum, ltilt) = tilt_sum(node.left.clone());
        let (rsum, rtilt) = tilt_sum(node.right.clone());
        (node.val + lsum + rsum, (lsum - rsum).abs() + ltilt + rtilt)
    } else { (0, 0) }
}

#[allow(dead_code)]
pub fn find_tilt(root: Tree) -> i32 {
    tilt_sum(root).1
}

#[derive(Copy, Clone)]
struct MaxDiff { max: i32, min: i32, diff: i32 }

impl MaxDiff {
    fn calc(root: Tree) -> Option<Self> {
        let r = root?;
        let node = r.borrow();
        let left = Self::calc(node.left.clone());
        let right = Self::calc(node.right.clone());
        let children: Vec<_> = left.into_iter().chain(right.into_iter()).collect();
        let min = children.iter().fold(node.val, |x, c| x.min(c.min));
        let max = children.iter().fold(node.val, |x, c| x.max(c.max));
        let my_diff = (node.val - min).max(max - node.val);
        let diff = children.iter().fold(my_diff, |x, c| x.max(c.diff));

        Some(MaxDiff { max, min, diff })
    }
}

#[allow(dead_code)]
pub fn max_ancestor_diff(root: Tree) -> i32 {
    MaxDiff::calc(root).map(|x| x.diff).unwrap_or(0)
}

#[allow(dead_code)]
pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    a.into_iter().map(|v| v.into_iter().rev().map(|i| 1 - i).collect()).collect()
}

use std::collections::HashMap;

#[allow(dead_code)]
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map = HashMap::new();
    for num in nums {
        let count = map.remove(&num).unwrap_or(0) + 1;
        map.insert(num, count);
    }
    let mut items: Vec<_> = map.into_iter().map(|(k, v)| (k as i8, v as u8)).collect();

    let mut cache: Cache = HashMap::new();
    permutes(&mut items, &mut cache)
        .into_iter().map(|v| v.into_iter().map(|n| n as i32).collect()).collect()
}

type Cache = HashMap<Vec<(i8, u8)>, Vec<Vec<i8>>>;

fn permutes(nums: &mut Vec<(i8, u8)>, cache: &mut Cache) -> Vec<Vec<i8>> {
    if nums.iter().all(|&(_, i)| i == 0) { return vec![vec![]]; }
    if let Some(v) = cache.get(nums) { return v.clone(); };
    let mut res = vec![];
    for k in 0..nums.len() {
        if nums[k].1 == 0 { continue; }
        nums[k].1 -= 1;
        for mut v in permutes(nums, cache) {
            v.push(nums[k].0);
            res.push(v)
        }
        nums[k].1 += 1;
    }
    cache.insert(nums.clone(), res.clone());
    res
}

#[allow(dead_code)]
pub fn init_comb() -> Vec<Vec<u64>> {
    (0..32).scan(vec![], |cur, _| {
        *cur = cur.iter().scan(0, |p, i| {
            let res = *i + *p;
            *p = *i;
            Some(res)
        }).collect();
        cur.push(1);
        Some(cur.clone())
    }).collect()
}

#[allow(dead_code)]
pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
    (0..).map(|i| (1 + (minutes_to_test / minutes_to_die) as u64).pow(i) ).position(
        |x| x >= buckets as u64
    ).unwrap_or(0) as i32
}
