use std::iter::successors;

pub fn max_nice_divisors(n: i32) -> i32 {
    if n < 5 {
        return n;
    }
    let mult = |x: u64, y: u64| (x * y) % 1_000_000_007;
    let p3 = |t: i32| fast_pow(1, 3, (t / 3) as u32, mult);
    (match n % 3 {
        0 => p3(n),
        1 => mult(p3(n - 4), 4),
        _ => mult(p3(n - 2), 2),
    }) as i32
}

fn fast_pow<A: Clone, F>(one: A, a: A, pow: u32, f: F) -> A
where
    F: Fn(A, A) -> A,
{
    successors(Some(a), |m| Some(f(m.clone(), m.clone())))
        .enumerate()
        .take(32)
        .fold(one, |acc, (i, mul)| if pow & (1 << i) != 0 { f(acc, mul) } else { acc })
}
