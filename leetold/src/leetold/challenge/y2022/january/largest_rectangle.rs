pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let left = dists_left(&heights);
    let right = dists_left(heights.iter().rev());
    let triples = left.into_iter().zip(right.into_iter().rev()).zip(heights);
    triples.map(|((l, r), h)| h * (l + r - 1) as i32).max().unwrap_or(0)
}

fn dists_left<'a>(heights: impl IntoIterator<Item = &'a i32>) -> Vec<usize> {
    let mut stack = vec![];
    let dist = |(i, &h)| {
        while stack.last().filter(|&&(_, p)| p >= h).is_some() {
            stack.pop();
        }
        let l = stack.last().map(|&(j, _)| i - j).unwrap_or(i + 1);
        stack.push((i, h));
        l
    };
    heights.into_iter().enumerate().map(dist).collect()
}
