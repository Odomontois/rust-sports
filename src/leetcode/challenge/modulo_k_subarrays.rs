pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
    use std::collections::HashMap;

    let mut counts = HashMap::new();
    counts.insert(0, 1i64);
    let mut cur = 0;
    let mut res = 0;
    for num in nums {
        if num % modulo == k {
            cur = (cur + 1) % modulo;
        }
        let need = (cur - k + modulo) % modulo;
        res += counts.get(&need).unwrap_or(&0);
        *counts.entry(cur).or_insert(0) += 1;
    }
    res
}
