use std::{iter::repeat, ops::Range};

pub fn sub_str_hash(s: impl AsRef<str>, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
    let s = s.as_ref();
    let fen = Fen::new(modulo, power, s);
    let k = k as usize;
    for i in 0..s.len() - k + 1 {
        if fen.calc(i..i + k) == hash_value {
            return s[i..i + k].to_string();
        }
    }
    return String::new();
}

struct Fen {
    base: i64,
    pow: i64,
    end: usize,
    v: Vec<i32>,
}

struct Val {
    hash: i32,
    range: Range<usize>,
}

impl Fen {
    fn mod_pow(&self, mut p: usize) -> i64 {
        let mut acc = 1;
        let mut m = self.pow;
        while p > 0 {
            if p & 1 > 0 {
                acc = (acc * m) % self.base
            }
            p /= 2;
            m = (m * m) % self.base;
        }
        acc
    }

    fn empty(&self, range: Range<usize>) -> Val {
        Val { hash: 0, range }
    }

    fn base(&self, c: i32, i: usize) -> Val {
        Val {
            hash: (c as i64 % self.base) as i32,
            range: i..i + 1,
        }
    }

    fn combine(&self, v1: Val, v2: Val) -> Val {
        let pn = self.mod_pow(v1.range.len());
        Val {
            hash: ((v1.hash as i64 + v2.hash as i64 * pn) % self.base) as i32,
            range: v1.range.start..v2.range.end,
        }
    }

    fn new(base: i32, pow: i32, vals: &str) -> Self {
        let bs = vals.as_bytes();
        let mut fen = Fen {
            base: base as i64,
            pow: pow as i64,
            end: vals.len(),
            v: vec![],
        };
        fen.init_iter(0, 0..vals.len(), |i| bs[i] as i32 - 'a' as i32 + 1);
        fen
    }

    fn init_iter(&mut self, cur: usize, range: Range<usize>, vals: impl Fn(usize) -> i32 + Copy) -> Val {
        if self.v.len() <= cur {
            self.v.extend(repeat(0).take(cur + 1 - self.v.len()))
        }
        let res = if range.len() == 1 {
            self.base(vals(range.start), range.start)
        } else {
            let m = (range.start + range.end) / 2;
            let v1 = self.init_iter(cur * 2 + 1, range.start..m, vals);
            let v2 = self.init_iter(cur * 2 + 2, m..range.end, vals);
            self.combine(v1, v2)
        };
        self.v[cur] = res.hash;
        return res;
    }

    fn calc(&self, search: Range<usize>) -> i32 {
        self.calc_iter(0, 0..self.end, search).hash
    }

    fn calc_iter(&self, cur: usize, range: Range<usize>, search: Range<usize>) -> Val {
        if range == search {
            Val {
                hash: self.v[cur],
                range,
            }
        } else if search.is_empty() {
            self.empty(search)
        } else {
            let m = (range.start + range.end) / 2;
            let v1 = self.calc_iter(cur * 2 + 1, range.start..m, search.start..m.min(search.end));
            let v2 = self.calc_iter(cur * 2 + 2, m..range.end, m.max(search.start)..search.end);
            self.combine(v1, v2)
        }
    }
}

#[test]
fn test1() {
    assert_eq!("ee", &sub_str_hash("leetcode", 7, 20, 2, 0));
}

#[test]
fn test2() {
    assert_eq!("fbx", &sub_str_hash("fbxzaad", 31, 100, 3, 32));
}
