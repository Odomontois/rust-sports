pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
    let divs: Vec<_> = (1..=k).filter(|&x| k % x == 0).collect();
    let comp = |i: usize| {
        (0..divs.len())
            .filter(|&j| (divs[i] as i64 * divs[j] as i64) % k as i64 == 0)
            .collect()
    };
    let comps: Vec<Vec<usize>> = (0..divs.len()).map(comp).collect();
    let mut counts: Vec<i64> = vec![0; divs.len()];
    let mut res = 0;
    for x in nums {
        let i = divs.binary_search(&gcd(x, k)).unwrap_or(0);
        res += counts[i];
        for &j in &comps[i] {
            counts[j] += 1;
        }
    }
    res
}

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

#[test]
fn test1() {
    assert_eq!(7, count_pairs(vec![1, 2, 3, 4, 5], 2));
}

#[test]
fn test2() {
    assert_eq!(0, count_pairs(vec![1, 2, 3, 4], 5));
}
