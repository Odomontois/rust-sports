use std::ops::Deref;

pub fn is_subsequence(s: impl Deref<Target = str>, t: impl Deref<Target = str>) -> bool {
    let mut t = t.chars();
    s.chars().all(|c| t.by_ref().skip_while(|&x| x != c).next().is_some())
}

#[test]
fn test1() {
    assert!(is_subsequence("abc", "ahbgdc"))
}

#[test]
fn test2() {
    assert!(!is_subsequence("axc", "ahbgdc"))
}
