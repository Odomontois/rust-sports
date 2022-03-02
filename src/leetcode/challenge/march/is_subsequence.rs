pub fn is_subsequence(s: impl AsRef<str>, t: impl AsRef<str>) -> bool {
    let mut t = t.as_ref().chars();
    s.as_ref().chars().all(|c| t.by_ref().skip_while(|&x| x != c).is_some())
}

#[test]
fn test1(){
    assert!(is_subsequence("abc", "ahbgdc"))
}

#[test]
fn test2(){
    assert!(!is_subsequence("axc", "ahbgdc"))
}
