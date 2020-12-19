use crate::leetcode::data::Tree;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::hash::Hash;

pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
    let mut i = nums.iter().take_while(|&&x| x < 0).count();
    if i == 0 { return nums.into_iter().map(|x| x.pow(2)).collect(); }
    if i == nums.len() { return nums.into_iter().rev().map(|x| x.pow(2)).collect(); }
    let mut j = i;
    let mut res = Vec::new();
    while i > 0 && j < nums.len() {
        if nums[i - 1].abs() < nums[j].abs() {
            i -= 1;
            res.push(nums[i].pow(2));
        } else {
            res.push(nums[j].pow(2));
            j += 1
        }
    }
    if i == 0 { res.extend(nums.drain(j..).map(|x| x.pow(2))) }
    if j == nums.len() { res.extend(nums.drain(..i).rev().map(|x| x.pow(2))) }
    res
}


pub fn is_valid_bst(root: Tree) -> bool { left_right_most(root).is_some() }

fn left_right_most(root: Tree) -> Option<Option<(i32, i32)>> {
    let tr = if let Some(rc) = root { rc } else { return Some(None); };
    let t = tr.borrow();
    let left = if let Some((ll, lr)) = left_right_most(t.left.clone())? {
        if lr >= t.val { return None; }
        ll
    } else { t.val };
    let right = if let Some((rl, rr)) = left_right_most(t.right.clone())? {
        if rl <= t.val { return None; }
        rr
    } else { t.val };

    Some(Some((left, right)))
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    #[test]
    fn check() {
        println!("{} {} {}", size_of::<(i32, i32)>(), size_of::<Option<(i32, i32)>>(), size_of::<Option<Option<(i32, i32)>>>())
    }
}

pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
    sum_count_4([a, b, c, d], 0) as i32
}

pub fn sum_count_4<A: Add<A, Output=A> + Sub<A, Output=A> + Eq + Hash + Clone>(a: [Vec<A>; 4], tgt: A) -> u64 {
    let ab = build_sums(&a[0], &a[1]);
    let cd = build_sums(&a[2], &a[3]);
    ab.into_iter().filter_map(|(x, c1)|
        cd.get(&(tgt.clone() - x)).map(|&c2| c1 as u64 * c2 as u64)
    ).sum()
}

fn build_sums<A: Add<A, Output=A> + Eq + Hash + Clone>(a: &Vec<A>, b: &Vec<A>) -> HashMap<A, usize> {
    let mut res = HashMap::new();
    for x in a {
        for y in b {
            let u = x.clone() + y.clone();
            if let Some(c) = res.get_mut(&u) { *c += 1; } else {
                res.insert(u, 1);
            }
        }
    }
    res
}

//
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    nums.into_iter().scan((None, None), |(m1, m2), n| {
        if m2.iter().any(|&m| m < n) { return Some(Some(())); }
        if m1.iter().any(|&m| m < n) && m2.iter().all(|&m| m > n) { *m2 = Some(n) }
        if m1.iter().all(|&m| m > n) { *m1 = Some(n) }
        Some(None)
    }).flatten().next().is_some()
}


#[test]
fn kek() {
    println!("{}", None < Some(2))
}

pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let mut lines = grid.into_iter();
    let first = if let Some(f) = lines.next() { f } else { return 0; };
    let n = first.len();
    let mut init = vec![vec![-1000_000_000; n]; n];
    let ixs = |i: usize| i.max(1) - 1..(i + 2).min(n);
    init[0][first.len() - 1] = first[0] + first.last().copied().unwrap_or(0);
    lines.fold(init, |prev, line|
        (0..n).map(|i| (0..n).map(|j| {
            let profit = if i == j { line[i] } else { line[i] + line[j] };
            ixs(i)
                .filter_map(|ip| ixs(j).map(|jp| prev[ip][jp] + profit).max())
                .max().unwrap_or(0)
        }
        ).collect()).collect(),
    ).into_iter().filter_map(|v| v.into_iter().max()).max().unwrap_or(0)
}

#[test]
fn cherry_test() {
    fn check<'a, A>(xs: &'a [A]) where &'a A: IntoIterator<Item=&'a i32> {
        let input = xs.iter().map(|v| v.into_iter().copied().collect()).collect();
        println!("{}", cherry_pickup(input));
    }
    check(&[[3, 1, 1], [2, 5, 1], [1, 5, 5], [2, 1, 1]]);
    check(&[[1, 0, 0, 0, 0, 0, 1], [2, 0, 0, 0, 0, 3, 0], [2, 0, 9, 0, 0, 0, 0], [0, 3, 0, 5, 4, 0, 0], [1, 0, 2, 3, 0, 0, 6]]);
    check(&[[1, 0, 0, 3], [0, 0, 0, 3], [0, 0, 3, 3], [9, 0, 3, 3]]);
    check(&[[1, 1], [1, 1]]);
}


