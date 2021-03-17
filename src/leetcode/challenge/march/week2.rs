use std::{iter::successors, mem::swap};

use crate::leetcode::data::List;

#[cfg(test)]


pub fn has_all_codes(s: String, k: i32) -> bool {
    let mask = (1 << k) - 1;
    let mut seen = vec![false; 1 << k as usize];
    let (rem, _) = s.bytes().enumerate().fold((1 << k, 0), |(remains, code), (i, c)| {
        let next = mask & (code << 1) | (c as usize - '0' as usize);
        let reduce = i >= k as usize - 1 && !seen[next];
        seen[next] |= reduce;
        (remains - reduce as i32, next)
    });
    rem == 0
}

#[test]
fn test_all_codes() {
    assert!(has_all_codes("00110110".to_string(), 2));
    assert!(has_all_codes("00110".to_string(), 2));
    assert!(has_all_codes("0110".to_string(), 1));
    assert!(!has_all_codes("0110".to_string(), 2));
    assert!(!has_all_codes("0000000001011100".to_string(), 4));
    assert!(!has_all_codes("0000000001011100".to_string(), 4));
}

pub fn num_factored_binary_trees(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    const MOD: u64 = 1_000_000_007;
    let mut res = vec![1u64; nums.len()];
    for i in 0..nums.len() {
        res[i] = (1
            + (0..i)
                .filter(|&j| nums[i] % nums[j] == 0)
                .map(|j| (res[j] * nums.binary_search(&(nums[i] / nums[j])).map(|k| res[k]).unwrap_or(0)) % MOD)
                .sum::<u64>())
            % MOD;
    }
    (res.into_iter().sum::<u64>() % MOD) as i32
}

pub fn swap_nodes1(mut head: List, k: i32) -> List {
    let l = successors(head.as_ref(), |&x| x.next.as_ref()).count();
    let mut stash = None::<&mut i32>;
    let mut cur = head.as_mut();
    let (a, b) = (k - 1, l as i32 - k);
    let mut i = 0;
    while let Some(x) = cur {
        if i == a || i == b {
            if let Some(prev) = stash {
                swap(&mut x.val, prev);
                break;
            }
            stash = Some(&mut x.val)
        }
        i += 1;
        cur = x.next.as_mut()
    }
    head
}

pub fn swap_nodes(mut head: List, k: i32) -> List {
    let l = successors(head.as_ref(), |&x| x.next.as_ref()).count();
    let mut stash = None::<&mut i32>;
    let mut cur = head.as_mut();
    let (a, b) = (k - 1, l as i32 - k);
    let mut i = 0;
    while let Some(x) = cur {
        if i == a || i == b {
            if let Some(prev) = stash {
                swap(&mut x.val, prev);
                break;
            }
            stash = Some(&mut x.val)
        }
        i += 1;
        cur = x.next.as_mut()
    }
    head
} 


