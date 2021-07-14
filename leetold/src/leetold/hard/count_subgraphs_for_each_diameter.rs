pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    const D: usize = 1000_000;
    let mut dists = vec![vec![D; n]; n];
    for i in 0..n { dists[i][i] = 0 }
    let edges: Vec<_> = edges.into_iter().map(|v| [v[0] as usize - 1, v[1] as usize - 1]).collect();
    for &[x, y] in &edges {
        for a in 0..n {
            for b in 0..n {
                if dists[a][x] != D && dists[y][b] != D && dists[a][b] == D {
                    dists[a][b] = dists[a][x] + dists[y][b] + 1;
                    dists[b][a] = dists[a][b];
                }
            }
        }
    }
    let mut seen = [false; 16];
    let mut dist_count = vec![0; n - 1];

    for m in 1i32..(1 << edges.len()) {
        for i in 0..n { seen[i] = false }
        for (i, &[x, y]) in edges.iter().enumerate() {
            if m & (1 << i) == 0 { continue; }
            seen[x] = true;
            seen[y] = true;
        }
        let seen_elems = |from: usize| (from..n).filter(|&i| seen[i]);
        if seen_elems(0).count() != m.count_ones() as usize + 1 { continue; }
        let dist = seen_elems(0).filter_map(|i| seen_elems(i + 1).map(|j| dists[i][j]).max()).max().unwrap_or(0);
        dist_count[dist as usize - 1] += 1;
    }
    dist_count
}

#[test]
fn test() {
    fn check(xs: &[[i32; 2]], exp: &[i32]) {
        assert_eq!(
            count_subgraphs_for_each_diameter(
                (xs.len() + 1) as i32,
                xs.iter().map(|v| v.iter().copied().collect()).collect()),
            exp.iter().copied().collect::<Vec<_>>()
        )
    }
    check(&[[1, 2], [2, 3], [2, 4]], &[3, 4, 0]);
    check(&[[1, 3], [1, 4], [2, 3]], &[3, 2, 1]);
    check(&[[1, 5], [2, 3], [2, 4], [2, 5]], &[4, 5, 3, 0]);
}


