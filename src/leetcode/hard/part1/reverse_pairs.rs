use crate::data::fenwick::FenSum;

pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
    let mut sorted: Vec<_> = nums.iter().map(|&x| x as i64).collect();
    sorted.sort();
    let mut fen = FenSum::new(0, nums.len());
    nums.into_iter().map(|x| {
        let before = sorted.binary_search(&(2 * x as i64 + 1)).unwrap_or_else(|x| x);
        let mine = sorted.binary_search(&(x as i64)).unwrap();
        let res = fen.get(before..).unwrap_or(0);
        fen.increment(mine, 1);
        res
    }).sum()
}


#[test]
fn rev_test() {
    fn check(xs: &[i32], exp: i32) { assert_eq!(reverse_pairs(xs.to_vec()), exp) }
    check(&[1, 3, 2, 3, 1], 2);
    check(&[2, 4, 3, 5, 1], 3);
    check(&[233, 2000000001, 234, 2000000006, 235, 2000000003, 236, 2000000007, 237, 2000000002, 2000000005, 233, 233, 233, 233, 233, 2000000004], 40)
}



