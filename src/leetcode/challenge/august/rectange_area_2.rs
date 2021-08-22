pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1000_000_007;
    let to_arr = |v: Vec<i32>| [[v[0], v[1]], [v[2] + 1, v[3] + 1]];
    let rectangles: Vec<_> = rectangles.into_iter().map(to_arr).collect();
    let coords = |i: usize| {
        let mut arr: Vec<_> = rectangles.iter().flat_map(|v| vec![v[0][i], v[1][i]]).collect();
        arr.sort();
        arr.dedup();
        arr
    };
    let [xs, ys] = [coords(0), coords(1)];
    let mut include = vec![vec![false; ys.len() - 1]; xs.len() - 1];
    let find = |v: &Vec<i32>, x: i32| v.binary_search(&x).unwrap();
    for [[sx, sy], [ex, ey]] in rectangles {
        let [[si, sj], [ei, ej]] = [[find(&xs, sx), find(&ys, sy)], [find(&xs, ex), find(&ys, ey)]];
        for i in si..ei {
            for j in sj..ej {
                include[i][j] = true;
            }
        }
    }

    let calc_one = |i, j, d| d as i64 * (xs[i + 1] - xs[i]) as i64 * (ys[j + 1] - ys[j]) as i64;
    let sum_row = |(i, v): (usize, Vec<bool>)| v.into_iter().enumerate().map(|(j, d)| calc_one(i, j, d)).sum::<i64>();
    (include.into_iter().enumerate().map(sum_row).sum::<i64>() % MOD) as i32
}
