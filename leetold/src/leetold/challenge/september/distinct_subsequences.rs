pub fn num_distinct(s: String, t: String) -> i32 {
    let mut idxs = <[Vec<_>; 26]>::default();
    let code = |c| c as u8 as usize - 'a' as u8 as usize;
    for (i, c) in t.to_lowercase().chars().map(code).enumerate() {
        idxs[c].push(i)
    }
    let mut counts = vec![0; t.len() + 1];
    counts[0] = 1;
    for c in s.to_lowercase().chars() {
        for &i in idxs[code(c)].iter().rev() {
            counts[i + 1] += counts[i]
        }
    }

    counts[t.len()]
}

#[cfg(test)]
fn check(s: &str, t: &str, exp: i32) {
    let act = num_distinct(s.to_string(), t.to_owned());
    assert_eq!(exp, act, "searching {} in {}, expected {}, got {}", t, s, exp, act)
}

#[test]
fn test1() {
    check("rabbbit", "rabbit", 3);
}

#[test]
fn test2() {
    check("babgbag", "bag", 5);
}
