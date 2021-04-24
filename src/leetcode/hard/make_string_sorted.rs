use crate::data::modnum::ModNum;
use std::iter::once;

lazy_static! {
    static ref FACT: Vec<ModNum> = once(1i64)
        .chain(1..)
        .map(ModNum)
        .scan(ModNum(1), |n, x| {
            *n = *n * x;
            Some(*n)
        })
        .take(5000)
        .collect();
}

pub fn make_string_sorted(s: String) -> i32 {
    s.chars()
        .rev()
        .scan(Denom::default(), |denom, c| {
            denom.put(c, 1);
            Some(denom.up_to(c))
        })
        .fold(ModNum(0), |x, y| x + y)
        .0 as i32
}
#[derive(Default, Debug)]
struct Denom {
    count: usize,
    chars: [i64; 26],
    denom: ModNum,
}

impl Denom {
    fn put(&mut self, c: char, amt: i64) {
        let c = c as usize - 'a' as usize;
        let t = self.chars[c];
        self.denom = self.denom / FACT[t as usize] * FACT[(t + amt) as usize];
        self.chars[c] += amt;
        self.count = (self.count as i64 + amt) as usize;
    }
    fn perms(&self) -> ModNum {
        FACT[self.count] / self.denom
    }
    fn up_to(&mut self, c: char) -> ModNum {
        let mut res = ModNum(0);
        for x in 0..c as usize - 'a' as usize {
            if self.chars[x] == 0 {
                continue;
            }
            self.put((x + 'a' as usize) as u8 as char, -1);
            res = res + self.perms();
            self.put((x + 'a' as usize) as u8 as char, 1);
        }
        res
    }
}

#[test]
fn check() {
    fn test(s: &str, exp: i32) {
        assert_eq!(make_string_sorted(s.to_string()), exp);
    }
    test("abc", 0);
    test("cba", 5);
    test("aabaa", 2);
    test("cdbea", 63);
    test("leetcodeleetcodeleetcode", 982157772);
}
