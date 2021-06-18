use std::iter::successors;

fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut fibs = successors(Some((0, 1)), |&(a, b)| Some((b, a + b)));
    let (a, b) = fibs.find(|&(a, b)| a * b >= prod).unwrap_or((0, 1));
    (a, b, a * b == prod)
}
