type Counts = Vec<i64>;
const MOD: i64 = 1_000_000_007;
pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
    let divs: Vec<Vec<_>> = (0..=max_value as usize)
        .map(|k| {
            (1..)
                .take_while(|i| i * i <= k)
                .filter(|i| k % i == 0)
                .flat_map(|i| if i * i == k { vec![i] } else { vec![i, k / i] })
                .collect()
        })
        .collect();

    let one: Counts = (0..divs.len()).map(|_| 1).collect();
    let zero: Counts = (0..divs.len()).map(|i| if i == 1 { 1 } else { 0 }).collect();
    let mul = |x: &Counts, y: &Counts| {
        divs.iter()
            .enumerate()
            .map(|(a, ds)| ds.iter().map(|&b| (x[b] * y[a / b]) % MOD).sum::<i64>() % MOD)
            .collect()
    };
    let s: i64 = fast_pow(zero, one, n, mul).iter().sum();
    (s % MOD) as i32
}

fn fast_pow<A>(one: A, mut x: A, pow: i32, mul: impl Fn(&A, &A) -> A) -> A {
    let bits = (0..)
        .take(32 - pow.leading_zeros() as usize)
        .map(|i| pow & (1 << i) != 0);
    let mut res = one;
    for b in bits {
        if b {
            res = mul(&res, &x);
        }
        x = mul(&x, &x);
    }
    res
}

#[test]
fn test1() {
    assert_eq!(10, ideal_arrays(2, 5))
}

#[test]
fn test2() {
    assert_eq!(11, ideal_arrays(5, 3))
}

#[test]
fn test3() {
    assert_eq!(19, ideal_arrays(4, 4))
}

#[test]
fn check1() {
    ideal_arrays(10000, 10000);
}
