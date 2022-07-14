pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
    let revs: Vec<_> = min_from(nums.iter().rev()).map(|i| nums.len() - i - 1).collect();
    min_from(nums.iter())
        .zip(revs.into_iter().rev())
        .zip(&nums)
        .filter_map(|((from, to), &x)| {
            let k = to - from + 1;
            (x as usize * k > threshold as usize).then(|| k as i32)
        })
        .next()
        .unwrap_or(-1)
}

fn min_from<A: Ord>(nums: impl Iterator<Item = A>) -> impl Iterator<Item = usize> {
    nums.enumerate().scan(Vec::<(usize, A)>::new(), |stack, (i, x)| {
        while stack.last().into_iter().any(|(_, y)| *y >= x) {
            stack.pop();
        }
        let from = stack.last().map(|(i, _)| *i + 1).unwrap_or(0);
        stack.push((i, x));
        Some(from)
    })
}
