pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
    let mut diffs: Vec<_> = nums1.iter().zip(&nums2).map(|(x, y)| (x - y).abs()).collect();
    diffs.sort();
    let k = (k1 + k2) as i64;

    let mut low = -1;
    let mut high = diffs.iter().copied().max().unwrap_or(0) + 1;
    let mut best: i64 = diffs.iter().map(|x| (x * x) as i64).sum();
    while high - low > 1 {
        let m = (high + low) / 2;
        let s: i64 = diffs.iter().map(|&x| (if x > m { x - m } else { 0 }) as i64).sum();
        if s > k {
            low = m
        } else {
            let rem = (k - s) as usize;
            high = m;
            let m = m as i64;
            best = diffs
                .iter()
                .rev()
                .enumerate()
                .map(|(i, x)| {
                    let x = *x as i64;
                    let z = if i < rem {
                        (m - 1).max(0)
                    } else if m < x {
                        m
                    } else {
                        x
                    };
                    z * z
                })
                .sum();
        }
    }
    best
}


#[test]
fn test3(){
    assert_eq!(43, min_sum_square_diff(vec![1,4,10,12], vec![5,8,6,9], 1, 1))
}