pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
    arr.windows(2)
        .map(|v| (v[0] - v[1]).signum())
        .fold((0, 0, 0), |(cnt, ps, mx), s| {
            let cnt = if ps * s == -1 { cnt + 1 } else { s.abs() };
            (cnt, s, mx.max(cnt))
        })
        .2 + 1
}
