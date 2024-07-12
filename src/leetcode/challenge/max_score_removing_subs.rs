pub fn maximum_gain(s: impl AsRef<str>, x: i32, y: i32) -> i32 {
    if x > y {
        gain::<b'a', b'b'>(x, y, s.as_ref())
    } else {
        gain::<b'b', b'a'>(y, x, s.as_ref())
    }
}

fn gain<const A: u8, const B: u8>(x: i32, y: i32, s: &str) -> i32 {
    let (mut cb, mut ca, mut res) = (0, 0, 0);
    for c in s.bytes() {
        if c == B && ca > 0 {
            ca -= 1;
            res += x;
        } else if c == B {
            cb += 1;
        } else if c == A {
            ca += 1;
        } else {
            res += y * ca.min(cb);
            (ca, cb) = (0, 0);
        }
    }
    res + y * ca.min(cb)
}

#[test]
fn ex1() {
    assert_eq!(19, maximum_gain("cdbcbbaaabab", 4, 5))
}

#[test]
fn ex2() {
    assert_eq!(20, maximum_gain("aabbaaxybbaabb", 5, 4))
}

#[test]
fn ex3() {
    assert_eq!(34, maximum_gain("ababababababab", 4, 5))
}

#[test]
fn wa1() {
    assert_eq!(23712, maximum_gain("cbaabwbbbabbwaaq", 4074, 9819))
}
