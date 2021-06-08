pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
    let minmax = |(mn, mx): (i32, i32), x| (mn.min(x), mx.max(x));
    let (from, to) = nums.iter().copied().fold((nums[0], nums[0]), minmax);
    let mut bit_set = vec![0u64; (to - from) as usize / 64 + 1];
    for num in nums {
        bit_set[(num - from) as usize / 64] |= 1 << (num - from) % 64;
    }

    (1..=to)
        .filter(|&x| {
            (1..=to / x)
                .filter(|&j| {
                    let u = x * j - from;
                    u >= 0 && bit_set[u as usize / 64] & (1 << u % 64) != 0
                })
                .fold(None, |p, x|  Some(gcd(p.unwrap_or(x), x)))
                == Some(1)
        })
        .count() as i32
}

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        return x;
    }
    gcd(y, x % y)
}

#[test]
fn test() {
    fn check(xs: &[i32], exp: i32) {
        assert_eq!(count_different_subsequence_gc_ds(xs.to_vec()), exp)
    }
    check(&[6, 10, 3], 5);
    check(&[5, 15, 40, 5, 6], 7);
}
