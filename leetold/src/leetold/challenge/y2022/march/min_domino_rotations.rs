pub fn min_domino_rotations2(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    let step = |p, s, x, y| (x == p).then(|| s).or((y == p).then(|| s + 1));
    let count = |p, v1: &[i32], v2: &[i32]| v1.iter().zip(v2).try_fold(0, |s, (&x, &y)| step(p, s, x, y));
    let rots = (1..=6).flat_map(|i| [count(i, &tops, &bottoms), count(i, &bottoms, &tops)]);
    rots.flatten().min().unwrap_or(-1)
}

pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    let n = tops.len();
    let mut counts = [[0; 3]; 6];
    for (x, y) in tops.into_iter().zip(bottoms) {
        counts[x as usize - 1][1] += 1;
        counts[y as usize - 1][2] += 1;
        counts[x as usize - 1][0] += 1;
        counts[y as usize - 1][0] += (x != y) as usize;
    }
    (0..5)
        .filter(|&i| counts[i][0] == n)
        .flat_map(|i| (1..=2).map(move |j| (n - counts[i][j]) as i32))
        .min()
        .unwrap_or(-1)
}
