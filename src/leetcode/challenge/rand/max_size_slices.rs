#[allow(dead_code)]
pub fn max_size_slices(slices: Vec<i32>) -> i32 {
    let n = slices.len();
    let mut dp = vec![vec![0; n + 2]; n + 2];
    for l in 0..n {
        for i in 0..(n - l) {
            let j = i + l;
            dp[i][i + l] = (i..j + 1).map(|k| cost(&dp, i, j, k) + slices[k]).max().unwrap_or(0);
        }
    }
    dp[0][n - 1]
}
#[allow(unreachable_code)]
fn cost(dp: &Vec<Vec<i32>>, i: usize, j: usize, k: usize) -> i32 {
    if j - i <= 3 {
        0
    } else if k == i {
        dp[i + 2][j - 1]
    } else if k == j {
        dp[i + 1][j - 2]
    } else if k == i + 1 {
        dp[i + 3][j]
    } else if k == j - 1 {
        dp[i][j - 3]
    } else if k == i + 2 {
        (dp[i][i] + dp[i + 4][j - 1]).max(unimplemented!())
    } else { unimplemented!() }
}
