use crate::data::combinations::Combinations;

pub fn minimum_incompatibility(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let n = nums.len() / k as usize;
    if n == 1 { return 0; }
    min_incompat(nums, n).unwrap_or(-1)
}

fn repetition(nums: &Vec<i32>, x: i32) -> bool {
    nums.iter().rev().fold((x, false), |(p, h), &i| (i, h || i == p)).1
}

fn min_incompat(mut nums: Vec<i32>, n: usize) -> Option<i32> {
    if nums.len() == n { return Some(nums[n - 1] - nums[0]).filter(|_| !repetition(&nums, -1)); }
    let x = nums.pop().unwrap_or(0);
    Combinations::new(nums.len(), n - 1).with_dual(|i| nums[i]).filter_map(|(c, rest)| {
        if repetition(&c, x) {
            return None;
        }
        let j = x - c[0];
        min_incompat(rest, n).map(|z| j + z)
    }).min()
}

#[test]
fn min_test() {
    println!("{}", minimum_incompatibility(vec![1, 2, 1, 4], 2));
    println!("{}", minimum_incompatibility(vec![6, 3, 8, 1, 3, 1, 2, 2], 4));
    println!("{}", minimum_incompatibility(vec![5, 3, 3, 6, 3, 3], 3));
}