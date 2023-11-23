use std::{cmp::Reverse, collections::BinaryHeap};

pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut h = BinaryHeap::new();
    let mut res = vec![];
    h.push((Reverse(nums1[0] + nums2[0]), 0, 0));
    for _ in 0..k {
        if let Some((_, i, j)) = h.pop() {
            res.push(vec![nums1[i], nums2[j]]);
            if j == 0 && i < nums1.len() - 1 {
                h.push((Reverse(nums1[i + 1] + nums2[j]), i + 1, j));
            }
            if j < nums2.len() - 1 {
                h.push((Reverse(nums1[i] + nums2[j + 1]), i, j + 1));
            }
        }
    }
    res
}
