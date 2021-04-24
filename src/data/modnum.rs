use std::ops::{Add, Div, Mul};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModNum(pub i64);

const MOD: i64 = 1_000_000_007;

impl Default for ModNum {
    fn default() -> Self {
        ModNum(1)
    }
}

impl Add for ModNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self((self.0 + rhs.0) % MOD)
    }
}

impl Mul for ModNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self((self.0 * rhs.0) % MOD)
    }
}

fn euclid(x: i64, y: i64) -> (i64, i64) {
    if x < y {
        let (u, v) = euclid(y, x);
        return (v, u);
    }
    if y == 0 {
        return (1, 0);
    }
    let (u, v) = euclid(y, x % y);
    (v, u - (x / y) * v)
}

pub fn recip(x: i64) -> i64 {
    let (_, k) = euclid(MOD, x);
    (k + MOD) as i64 % MOD
}

impl Div for ModNum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * Self(recip(rhs.0))
    }
}

#[test]
fn check() {
    for i in 1..1000 {
        for j in 1..1000 {
            assert_eq!(ModNum(i) / ModNum(j) * ModNum(j), ModNum(i), "{} {}", i, j);
        }
    }
}
