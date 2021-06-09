fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut common = 1;
    let reduced: Vec<_> = l
        .into_iter()
        .map(|(n, d)| {
            let k = gcd(n, d);
            let (n, d) = (n / k, d / k);
            common = lcm(common, d);
            (n, d)
        })
        .collect();

    reduced.into_iter().map(|(n, d)| (common / d * n, common)).collect()
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
