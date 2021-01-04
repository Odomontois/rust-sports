pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let vals: std::collections::BTreeSet<_> = nums.into_iter().collect();
    let res = |x: i32, m: i32| {
        let mut pref = 0;
        for i in (0..30).rev() {
            let scope = 1 << i;
            let bit = (x & scope) ^ scope;
            let rbit = bit ^ scope;
            let check = |b: i32| {
                let lower = pref + b;
                let upper = (lower + scope).min(m + 1);
                return upper >= lower && vals.range(lower..upper).next().is_some();
            };
            pref += if check(bit) { bit } else if check(rbit) { rbit } else { return -1; }
        }
        pref ^ x
    };
    queries.into_iter().map(|v| res(v[0], v[1])).collect()
}

#[test]
fn check() {
    fn check(nums: &[i32], queries: &[[i32; 2]], exp: &[i32]) {
        let nums1 = nums.to_vec();
        let queries1 = queries.iter().copied().map(|[x, y]| vec![x, y]).collect();
        assert_eq!(&*maximize_xor(nums1, queries1), exp);
    }

    check(&[0, 1, 2, 3, 4], &[[3, 1], [1, 3], [5, 6]], &[3, 3, 7]);
    check(&[5, 2, 4, 6, 6, 3], &[[12, 4], [8, 1], [6, 3]], &[15, -1, 5]);
}

pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    box_types.sort_by_key(|x| -x[1]);
    box_types.into_iter().fold((truck_size, 0), |(s, acc), p| ((s - p[0]).max(0), acc + p[1] * p[0].min(s))).1
}