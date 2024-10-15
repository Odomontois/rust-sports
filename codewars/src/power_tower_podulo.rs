use lazy_static::lazy_static;

lazy_static! {
    static ref PRIMES: Vec<u64> = calc_primes();
}

fn tower(base: u64, h: u64, m: u32) -> u32 {
    if m == 1 {
        return 0;
    }
    if h == 0 {
        return 1;
    }
    let m = m as u64;
    if h == 1 {
        return (base % m) as u32;
    }
    let p = phi(m);
    let exp = tower(base, h - 1, p as u32);
    let exp = if exp == 0 { base } else { exp as u64 };
    fast_pow(base, exp, m) as u32
}

fn calc_primes() -> Vec<u64> {
    let mut primes = vec![2];
    for num in (3..4000).step_by(2) {
        let mut ps = primes.iter().copied().take_while(|&q| q * q <= num);
        let is_prime = ps.all(|q| num % q != 0);
        if is_prime {
            primes.push(num);
        }
    }
    primes
}

fn phi(mut m: u64) -> u64 {
    let mut result = 1;
    for p in &*PRIMES {
        if p * p > m {
            break;
        }
        if m % p != 0 {
            continue;
        }
        result *= p - 1;
        m /= p;
        while m % p == 0 {
            result *= p;
            m /= p;
        }
    }
    if m > 1 {
        result *= m - 1;
    }
    result
}

fn fast_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut cur = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            cur = (cur * base as u64) % m;
        }
        exp /= 2;
        base = (base * base) % m;
    }
    cur
}

#[test]
fn check_tower() {
    assert_eq!(tower(28, 3, 25), 21);
    assert_eq!(tower(4, 3, 10), 6);
    assert_eq!(tower(2, 3, 100000), 16);
    assert_eq!(tower(4, 4, 80), 16);
    assert_eq!(tower(9, 3, 100), 89);
    assert_eq!(tower(16, 3, 100), 16);
    assert_eq!(tower(16, 3, 80), 16);
}

#[test]
fn check_phi() {
    assert_eq!(phi(543345), 253440);
    assert_eq!(phi(253440), 61440);
    assert_eq!(phi(61440), 16384);
    assert_eq!(phi(16384), 8192);
    assert_eq!(phi(8192), 4096);
    assert_eq!(phi(4096), 2048);
    assert_eq!(phi(2048), 1024);
    assert_eq!(phi(1024), 512);
    assert_eq!(phi(512), 256);
    assert_eq!(phi(256), 128);
    assert_eq!(phi(128), 64);
    assert_eq!(phi(64), 32);
    assert_eq!(phi(32), 16);
    assert_eq!(phi(16), 8);
    assert_eq!(phi(8), 4);
    assert_eq!(phi(4), 2);
    assert_eq!(phi(2), 1);
}
