use itertools::{iterate, Itertools};
use std::iter::from_fn;

fn thirt(n: i64) -> i64 {
    iterate(n, |&x| process(x))
        .tuple_windows()
        .take_while(|&(x, y)| x != y)
        .last()
        .unwrap_or((n, n))
        .1
}

fn process(mut n: i64) -> i64 {
    from_fn(move || {
        if n == 0 {
            return None;
        }
        let res = n % 10;
        n /= 10;
        Some(res)
    })
    .zip((&[1, 10, 9, 12, 3, 4]).iter().cycle())
    .map(|(d, &t)| d * t)
    .sum()
}
