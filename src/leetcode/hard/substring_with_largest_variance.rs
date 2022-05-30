use std::collections::HashSet;

use std::ops::Deref;

pub fn largest_variance(s: impl Deref<Target = str>) -> i32 {
    let chars: HashSet<_> = s.chars().collect();
    let mut res = 0;
    for &c1 in &chars {
        for &c2 in &chars {
            if c1 != c2 {
                res = res.max(max_variance(s.chars(), c1, c2));
            }
        }
    }
    res
}

fn max_variance<A: Eq>(elems: impl IntoIterator<Item = A>, low: A, high: A) -> i32 {
    let mut res = 0;
    let mut mv = 0;
    let mut mv_ex = None;
    let mut var = 0;
    for e in elems {
        if e == low {
            var -= 1;
            mv_ex = mv_ex.max(Some(mv));
            mv = mv.max(-var);
        } else if e == high {
            var += 1;
        }
        if let Some(q) = mv_ex {
            res = res.max(q + var)
        }
    }

    res
}

#[test]
fn test1() {
    assert_eq!(3, largest_variance("aababbb"))
}

#[test]
fn test2() {
    assert_eq!(0, largest_variance("abcde"))
}

#[test]
fn wa1() {
    assert_eq!(3, largest_variance("ieiii"))
}
