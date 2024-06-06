pub fn check_record(n: i32) -> i32 {
    let xx = fast_pow(ones(|i, j| i == j), ATTENDANCE, n as u64, mul);
    let walk = mul(xx, ones::<1, 6>(|_, i| i == 0));
    mul(ones::<6, 1>(|_, _| true), walk)[0][0] as i32
}

static ATTENDANCE: [[u64; 6]; 6] = [
    [1, 1, 1, 0, 0, 0],
    [1, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0],
    [1, 0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 1, 0],
];

fn ones<const M: usize, const N: usize>(p: impl Fn(usize, usize) -> bool) -> [[u64; M]; N] {
    let mut res = [[0; M]; N];
    for i in 0..M {
        for j in 0..N {
            res[j][i] = p(i, j) as u64;
        }
    }
    res
}

fn mul<const A: usize, const B: usize, const C: usize>(x: [[u64; B]; A], y: [[u64; C]; B]) -> [[u64; C]; A] {
    let mut z = [[0; C]; A];
    for i in 0..A {
        for j in 0..C {
            let zz = &mut z[i][j];
            for k in 0..B {
                *zz += x[i][k] * y[k][j];
            }
            *zz %= 1_000_000_007;
        }
    }
    z
}

fn fast_pow<A: Copy>(mut res: A, mut x: A, mut pow: u64, mul: impl Fn(A, A) -> A) -> A {
    while pow > 0 {
        if pow & 1 == 1 {
            res = mul(res, x);
        }
        x = mul(x, x);
        pow >>= 1;
    }
    res
}

