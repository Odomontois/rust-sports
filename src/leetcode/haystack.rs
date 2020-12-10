use crate::data::kmp::KMP;

#[allow(dead_code)]
pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() { return 0; }
    let n = needle.len();
    KMP::build(&needle).analyze(&haystack).position(|l| l == n).map(|x| (x + 1 - n) as i32).unwrap_or(-1)
}

