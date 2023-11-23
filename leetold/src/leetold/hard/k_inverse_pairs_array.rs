const MOD: i32 = 1_000_000_007;

pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
    let mut cur = vec![1];
    for u in 1..n {
        let sums = cur.iter().scan(0, |s, &x| {
            *s = (*s + x) % MOD;
            Some(*s)
        });
        let sums: Vec<_> = Some(0).into_iter().chain(sums).collect();
        let next = (0..=k.min(u * (u + 1) / 2)).map(|i| {
            let to = (sums.len() - 1).min(i as usize + 1);
            let from = 0.max(i - u) as usize;
            (sums[to] - sums[from] + MOD) % MOD
        });
        cur = next.collect();
        #[cfg(test)]
        {
            println!("{cur:?}")
        }
    }
    cur.get(k as usize).copied().unwrap_or(0)
}

#[test]
fn lol() {
    assert_eq!(1, k_inverse_pairs(10, 45))
}

#[test]
fn test1() {
    assert_eq!(1, k_inverse_pairs(4, 6))
}

#[test]
fn test2() {
    assert_eq!(98, k_inverse_pairs(7, 4))
}

#[test]
fn test3() {
    assert_eq!(10947079, k_inverse_pairs(20, 10))
}

#[test]
fn test4() {
    assert_eq!(369119900, k_inverse_pairs(40, 25))
}

#[test]
fn example1() {
    assert_eq!(1, k_inverse_pairs(3, 0))
}

#[test]
fn example2() {
    assert_eq!(2, k_inverse_pairs(3, 1))
}
