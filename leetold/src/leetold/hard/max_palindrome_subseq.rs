pub fn longest_palindrome(word1: String, word2: String) -> i32 {
    let s: &[char] = &word1.chars().chain(word2.chars()).collect::<Vec<_>>();
    let mut p = vec![vec![0; s.len()]; s.len()];
    for i in 0..p.len() {
        p[i][i] = 1
    }
    for k in 1..p.len() {
        for i in 0..p.len() - k {
            let j = i + k;
            p[i][j] = p[i + 1][j]
                .max(p[i][j - 1])
                .max((s[i] == s[j]) as usize * (p[i + 1][j - 1] + 2));
        }
    }
    let p: &[_] = &p;
    (0..word1.len())
        .flat_map(|i| {
            (word1.len()..word1.len() + word2.len())
                .filter(move |&j| s[i] == s[j])
                .map(move |j| 2 + p[i + 1][j - 1])
        })
        .max()
        .unwrap_or(0) as i32
}

#[test]
fn longest_check() {
    assert_eq!(longest_palindrome("cfe".to_string(), "ef".to_string()), 4)
}
