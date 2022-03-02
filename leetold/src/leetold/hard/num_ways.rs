// TLE
pub fn number_of_combinations(num: String) -> i32 {
    const MOD: i64 = 1000_000_007;
    let num: Vec<_> = num.chars().collect();
    if num[0] == '0' {
        return 0;
    }
    let mut res = vec![vec![1i64; num.len()]];

    for i in 1..num.len() {
        if num[i] == '0' {
            res.push(vec![0; num.len() - 1]);
            continue;
        }
        let line = |l: usize| {
            let x: i64 = (1..l.min(i + 1)).map(|k| res[i - k][k - 1]).sum();
            let y = if i >= l && num[i - l..i] <= num[i..i + l] {
                res[i - l][l - 1]
            } else {
                0
            };
            (x + y) % MOD
        };
        let line = (1..=num.len() - i).map(line).collect();
        res.push(line);
    }

    ((0..num.len()).map(|i| res[i][num.len() - i - 1]).sum::<i64>() % MOD) as i32
}


#[test]
fn test1() {
    assert_eq!(number_of_combinations("327".to_string()), 2);
}
#[test]
fn test2() {
    assert_eq!(number_of_combinations("094".to_string()), 0);
    assert_eq!(number_of_combinations("0".to_string()), 0);
}

#[test]
fn test3() {
    assert_eq!(number_of_combinations("9999999999999".to_string()), 101);
}
