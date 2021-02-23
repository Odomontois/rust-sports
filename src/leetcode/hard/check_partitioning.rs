pub fn check_partitioning(s: String) -> bool {
    let s: Vec<_> = s.chars().collect();
    let pals: Vec<_> = (0..2 * s.len() + 1)
        .map(|i| {
            let (mut b, mut e) = (i / 2, (i + 1) / 2);
            while b > 0 && e < s.len() && s[b - 1] == s[e] {
                b -= 1;
                e += 1;
            }
            e - b
        })
        .collect();

    for i in 1..s.len() - 1 {
        for j in i + 1..s.len() {
            if pals[i] >= i && pals[i + j] >= j - i && pals[j + s.len()] >= s.len() - j {
                return true;
            }
        }
    }
    false
}

#[test]
fn check_part() {
    assert!(check_partitioning("abcbdd".to_string()));
    assert!(!check_partitioning("bcbddxy".to_string()));
    assert!(check_partitioning("abcba".to_string()));
}
