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


