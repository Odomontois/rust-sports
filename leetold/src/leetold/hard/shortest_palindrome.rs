use crate::data::kmp::KMP;

pub fn shortest_palindrome(mut s: String) -> String {
    let n = s.len();
    if n <= 1 { return s; }
    let kmp = KMP::build(&s);
    s = s.chars().rev().collect();
    let add_len = kmp.analyze(&s).enumerate().filter_map(|(i, m)|
        if m + i + 1 == n {
            Some(n - 2 * m)
        } else if m + i + 2 == n {
            Some(n - 2 * m - 1)
        } else {
            // println!("{} {}", i, m);
            None
        }
    ).min().unwrap_or(n);
    let add: String = (&s[0..add_len]).chars().rev().collect();
    format!("{}{}", s, add)
}

#[test]
fn test_pal() {
    fn check(s: &str, s2: &str) { assert_eq!(shortest_palindrome(s.to_string()), s2.to_string()) }
    check("aba", "aba");
    check("abba", "abba");
    check("abb", "bbabb");
    check("bba", "abba");
    check("abc", "cbabc");
    check("ab", "bab");
    check("", "");
    check("a", "a");
    check("abcba", "abcba");
    check("abccba", "abccba");
}