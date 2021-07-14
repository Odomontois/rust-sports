#[allow(dead_code)]
pub fn decrypt(mut code: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 0 { return vec![0; code.len()]; }
    let n = code.len();
    code.append(&mut code.clone());
    let it = code.windows(k.abs() as usize).map(|vs| vs.iter().sum()).collect::<Vec<_>>().into_iter();
    if k < 0 { it.rev().skip(1).take(n).rev().collect() } else { it.into_iter().skip(1).take(n).collect() }
}

use std::iter::once;


#[allow(dead_code)]
pub fn minimum_deletions(s: String) -> i32 {
    let n = s.len();
    let bc: Vec<_> = once(0).chain(s.chars().scan(0, |count, c| {
        if c == 'b' { *count += 1 }
        Some(*count)
    })).collect();
    let ac: Vec<_> = once(0).chain(s.chars().rev().scan(0, |count, c| {
        if c == 'a' { *count += 1 }
        Some(*count)
    })).collect();
    (0..s.len() + 1).map(|i| bc[i] + ac[n - i]).min().unwrap_or(0)
}

use std::collections::{VecDeque};

#[allow(dead_code)]
pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
    let mut forwarded = vec![false; 1000_000 as usize];
    let mut backed = vec![false; 1000_000 as usize];
    for xf in forbidden {
        forwarded[xf as usize] = true;
        backed[xf as usize] = true;
    };
    let mut q = VecDeque::new();
    q.push_back((0, 0, false));
    while let Some((pos, steps, back)) = q.pop_front() {
        if pos == x { return steps; }
        let seen = if back { &mut backed } else { &mut forwarded };
        if pos < 0 || pos >= 1000_000 || seen[pos as usize] { continue; }
        seen[pos as usize] = true;
        q.push_back((pos + a, steps + 1, false));
        if !back { q.push_back((pos - b, steps + 1, true)); }
    }
    -1
}

use std::collections::{HashMap, BTreeSet};

#[allow(dead_code)]
pub fn can_distribute(nums: Vec<i32>, mut quantity: Vec<i32>) -> bool {
    let mut counts = HashMap::<i32, i32>::new();
    for n in nums { counts.insert(n, counts.get(&n).cloned().unwrap_or(0) + 1); }
    quantity.sort_by_key(|&x| -x);
    let mut cnt: Vec<_> = counts.values().cloned().collect();
    cnt.sort_by_key(|&x| -x);
    let mut set = cnt.into_iter().enumerate().map(|(i, n)| (n, i)).collect();

    distr(&mut set, quantity.as_slice())
}

fn distr(nums: &mut BTreeSet<(i32, usize)>, qs: &[i32]) -> bool {
    if qs.is_empty() { return true; };
    let q = qs[0];
    let vs: Vec<_> = nums.iter().rev().cloned().take_while(|(n, _)| *n >= q).collect();
    for (n, j) in vs {
        nums.remove(&(n, j));
        nums.insert((n - q, j));
        if distr(nums, &qs[1..]) { return true; }
        nums.remove(&(n - q, j));
        nums.insert((n, j));
    }
    false
}