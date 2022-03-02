use std::collections::HashMap;

pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    let m = counter(nums1, nums2);
    pairs(&nums3, &nums4).map(|s| m.get(&(-s)).copied().unwrap_or(0)).sum()
}

fn pairs<'a>(nums1: &'a [i32], nums2: &'a [i32]) -> impl Iterator<Item = i32> + 'a {
    nums1.iter().flat_map(move |x| nums2.iter().map(move |&y| x + y))
}

fn counter(nums1: Vec<i32>, nums2: Vec<i32>) -> HashMap<i32, i32> {
    let mut h = HashMap::new();
    pairs(&nums1, &nums2).for_each(|s| *h.entry(s).or_insert(0) += 1);
    h
}
