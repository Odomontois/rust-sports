use crate::data::radix_sort::*;

pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
    nums.radix_sort();
    nums.windows(2).map(|w| w[1] - w[0]).max().unwrap_or(0)
}



