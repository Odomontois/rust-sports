pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let m = matrix.first().map(|v| v.len()).unwrap_or(0);
    let mut paths = vec![vec![1; m]; n];
    let mut elems = matrix.iter().enumerate().flat_map(|(i, row)|
        row.iter().cloned().enumerate().map(|(j, x)| (x, i, j)).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    elems.sort();
    for (x, i, j) in elems {
        for &(u, v) in &[(2, 1), (0, 1), (1, 2), (1, 0)] {
            if i + u > 0 && i + u <= n && j + v > 0 && j + v <= m && x > matrix[i + u - 1][j + v - 1] {
                paths[i][j] = paths[i][j].max(paths[i + u - 1][j + v - 1] + 1);
            }
        }
    }

    paths.into_iter().filter_map(|row| row.into_iter().max()).max().unwrap_or(0)
}

#[test]
fn test() {
    fn check<'a, A>(mx: &'a [A], exp: i32) where &'a A: IntoIterator<Item=&'a i32> {
        let res = longest_increasing_path(mx.iter().map(|a| a.into_iter().cloned().collect()).collect());
        assert_eq!(res, exp)
    }
    check(&[[9, 9, 4], [6, 6, 8], [2, 1, 1]], 4);
    check(&[[3, 4, 5], [3, 2, 6], [2, 2, 1]], 4);
    check::<[i32; 0]>(&[], 0);
    check(&[[1],[2],[3],[2],[1]], 3);
}


