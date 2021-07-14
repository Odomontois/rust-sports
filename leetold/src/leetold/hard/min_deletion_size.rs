pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let n = strs[0].len();
    let strs: Vec<_> = strs.iter().map(|s| s.as_bytes()).collect();
    let mut res = vec![];
    for i in 0..n {
        let next = (0..i)
            .filter(|&j| strs.iter().all(|s| s[j] <= s[i]))
            .map(|j| i - j - 1 + res[j])
            .min()
            .unwrap_or(i);
        res.push(next)
    }

    res.into_iter()
        .enumerate()
        .map(|(i, r)| r + n - i - 1)
        .min()
        .unwrap_or(n) as i32
}

#[test]
fn test_mid_del() {
    check(&["babca", "bbazb"], 3);
    check(&["edcba"], 4);
    check(&["ghi", "def", "abc"], 0);
    check(&["aaababa", "ababbaa"], 4);
}
fn check(strs: &[&str], exp: i32) {
    assert_eq!(min_deletion_size(strs.iter().map(|s| s.to_string()).collect()), exp);
}
