use std::{collections::HashMap, iter::once};

pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut counts = HashMap::new();
    for num in nums {
        *counts.entry(num).or_insert(0) += 1
    }
    let mut res = vec![vec![]];
    for (k, m) in counts {
        let mut new = vec![];
        for c in 0..=m {
            for mut i in res.iter().cloned() {
                i.extend(once(k).cycle().take(c));
                new.push(i);
            }
        }
        res = new;
    }

    res
}
