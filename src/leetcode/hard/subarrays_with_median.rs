use std::{collections::HashMap, iter::once};

pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut balanceCounts: HashMap<_, _> = once((0, 1)).collect();
    let mut balance = 0;
    let mut res = 0;
    let mut found = false;

    for x in nums {
        balance += (x - k).signum();
        found |= x == k;
        if found {
            let count = |i: i32| balanceCounts.get(&i).copied().unwrap_or(0);
            res += count(balance) + count(balance - 1);
        } else {
            *balanceCounts.entry(balance).or_insert(0) += 1;
        }
    }
    res
}
