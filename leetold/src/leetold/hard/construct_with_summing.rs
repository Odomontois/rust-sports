// https://leetcode.com/problems/construct-target-array-with-multiple-sums/

use std::collections::BinaryHeap;

pub fn is_possible<A: AsRef<[i32]>>(target: A) -> bool {
    let mut sum: i64 = target.as_ref().iter().map(|x| *x as i64).sum();
    let mut nums: BinaryHeap<_> = target.as_ref().iter().copied().collect();
    while let Some(x) = nums.pop() {
        let x = x as i64;
        let part_sum = sum - x;
        if x == 1 || part_sum == 1 {
            return true;
        }
        let rem = x % part_sum.max(1);
        if x < part_sum || rem == 0 {
            return false;
        }
        nums.push(rem as i32);
        sum = sum - x + rem;
    }
    true
}

#[test]
fn example1() {
    assert!(is_possible([9, 3, 5]))
}

#[test]
fn example2() {
    assert!(!is_possible([1, 1, 1, 2]))
}

#[test]
fn example3() {
    assert!(is_possible([8, 5]))
}

#[test]
fn wa1(){
    assert!(!is_possible([2]))
}
