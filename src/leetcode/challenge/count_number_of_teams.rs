pub fn num_teams(rating: Vec<i32>) -> i32 {
    increasing_triples(&rating, |x, y| x < y) + increasing_triples(&rating, |x, y| x > y)
}

fn increasing_triples<A: Ord>(xs: &[A], f: impl Fn(&A, &A) -> bool) -> i32 {
    let mut doubles = Vec::with_capacity(xs.len());
    let mut triples = 0;
    for (i, x) in xs.iter().enumerate() {
        let mut double = 0;
        for (y, &d) in xs[..i].iter().zip(&doubles) {
            if f(y, x) {
                triples += d;
                double += 1;
            }
        }
        doubles.push(double);
    }
    triples
}
