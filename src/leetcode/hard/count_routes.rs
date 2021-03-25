pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
    const MOD: u64 = 1_000_000_007;
    let mut res = vec![vec![0u64; locations.len()]; fuel as usize + 1];
    res[0][start as usize] = 1;

    for f in 0..=fuel as usize {
        for (i, &x) in locations.iter().enumerate() {
            for (j, &y) in locations.iter().enumerate() {
                let d = (x - y).abs() as usize;
                if i != j && d <= f {
                    res[f][i] = (res[f][i] + res[f - d][j]) % MOD;
                }
            }
        }
    }

    (res.into_iter().map(|r| r[finish as usize]).sum::<u64>() % MOD) as i32
}
