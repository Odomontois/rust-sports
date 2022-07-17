const MOD: i64 = 1_000_000_007;
pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
    let (n, m) = (grid.len(), grid[0].len());
    let mut cells: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().map(move |(j, x)| (*x, i, j)))
        .collect();
    cells.sort_by_key(|t| t.0);
    let mut res = vec![vec![1; m]; n];

    let neighbors = |i: usize, j: usize, x: i32| {
        [(i, j + 1), (i + 1, j), (i + 2, j + 1), (i + 1, j + 2)]
            .iter()
            .filter_map(|&(i, j)| (i > 0 && j > 0 && i <= n && j <= m && grid[i - 1][j - 1] < x).then(|| (i - 1, j - 1)))
            .collect::<Vec<_>>()
    };
    for (x, i, j) in cells {
        for (u, v) in neighbors(i, j, x) {
            res[i][j] = (res[i][j] + res[u][v]) % MOD;
        }
    }

    (res.iter().flatten().sum::<i64>() % MOD) as i32
}
