use crate::data::kmp::KMP;
use std::collections::HashSet;

pub fn distinct_echo_substrings(text: String) -> i32 {
    let mut res = HashSet::new();
    for i in 0..text.len() - 1 {
        let kmp = KMP::build(&text[i..]);
        for j in 1..=(text.len() - i) / 2 {
            if kmp.index[j * 2 - 1] >= j {
                let k = 2 * j - kmp.index[j * 2 - 1];
                if k == 0 || j % k == 0 {
                    res.insert(&text[i..i + j]);
                }
            }
        }
    }
    res.len() as i32
}

#[test]
fn test() {
    fn check(s: &str) { println!("{}", distinct_echo_substrings(s.to_string())) }
    check("abcabcabc");
    check("leetcodeleetcode");
}