pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len() as i32;
    nums.into_iter()
        .scan(std::collections::VecDeque::new(), |q, x| {
            if Some(&x) != q.back() {
                q.push_back(x)
            }
            while q.front().into_iter().any(|&y| x - y >= n) {
                q.pop_front();
            }
            Some(n - q.len() as i32)
        })
        .min()
        .unwrap_or(0)
}
