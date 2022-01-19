use std::collections::HashSet;

pub fn possibly_equals(s1: String, s2: String) -> bool {
    let mut iter = Iter { seen: HashSet::new() };

    iter.go(s1.as_bytes(), s2.as_bytes(), 0, 0)
}

struct Iter {
    seen: HashSet<(usize, usize, i32)>,
}

impl Iter {
    fn go(&mut self, s1: &[u8], s2: &[u8], d: i32, n: i32) -> bool {
        if n == 0 && !self.seen.insert((s1.len(), s2.len(), d)) {
            return false;
        }
        let dn = d + n;
        let is_digit = |&&x: &&u8| (x as char).is_digit(10);
        if let Some(&a) = s1.first().filter(is_digit) {
            let a = (a - '0' as u8) as i32;
            n > 0 && self.go(&s1[1..], s2, d, 10 * n + a) || self.go(&s1[1..], s2, dn, a)
        } else if let Some(b) = s2.first().filter(is_digit) {
            let b = (b - '0' as u8) as i32;
            n < 0 && self.go(s1, &s2[1..], d, 10 * n - b) || self.go(s1, &s2[1..], dn, -b)
        } else if dn < 0 && !s1.is_empty() {
            self.go(&s1[1..], s2, dn + 1, 0)
        } else if dn > 0 && !s2.is_empty() {
            self.go(s1, &s2[1..], dn - 1, 0)
        } else if s1.is_empty() || s2.is_empty() {
            s1.is_empty() && s2.is_empty() && dn == 0
        } else {
            s1[0] == s2[0] && self.go(&s1[1..], &s2[1..], 0, 0)
        }
    }
}

#[cfg(test)]
fn check(s1: &str, s2: &str, res: bool) {
    assert_eq!(possibly_equals(s1.to_string(), s2.to_string()), res)
}

#[test]
fn test1() {
    check("i18n", "internationalization", true)
}
#[test]
fn test2() {
    check("l123e", "44", true);
}

#[test]
fn test3() {
    check("a5b", "b5b", false);
}

#[test]
fn test4() {
    check("a5b", "a5c", false);
}
#[test]
fn test7() {
    check("ab", "a2", false);
}

#[test]
fn test5() {
    check("a11a11", "a11b11", true)
}

#[test]
fn test6() {
    check(
        "f864f565f752f771f985f158f736f593f965f949",
        "f572f754f364f482f721f849f529f637f55",
        true,
    )
}
