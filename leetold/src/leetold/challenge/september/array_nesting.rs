pub fn array_nesting(nums: Vec<i32>) -> i32 {
    let mut seen = vec![false; nums.len()];
    let u = (0..nums.len()).map(|i| {
        Some(0).filter(|_| seen[i]).unwrap_or_else(|| {
            let it = std::iter::successors(Some(nums[i] as usize), |&j| Some(nums[j] as usize)).take_while(|&j| j != i);
            it.chain(Some(i)).inspect(|&j| seen[j] = true).count() as i32
        })
    });
    u.max().unwrap_or(0) as i32
}
