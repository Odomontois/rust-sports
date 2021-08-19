use std::{
    iter::from_fn,
    ops::{Deref, DerefMut},
};

pub fn min_window(s: String, t: String) -> String {
    let mut mw = MinWindow::new();
    for c in t.chars() {
        *mw.char(c) -= 1;
    }
    let (mut start, mut end, mut beg, mut fin) = (s.chars(), s.chars(), 0, 0);
    let go = || -> Option<Option<&str>> {
        if mw.need > 0 {
            *mw.char(end.next()?) += 1;
            fin += 1;
        } else {
            *mw.char(start.next()?) -= 1;
            beg += 1;
        }
        Some(Some(&s[beg..fin]).filter(|_| mw.need == 0))
    };
    from_fn(go).flatten().min_by_key(|s| s.len()).unwrap_or("").to_string()
}

#[cfg(test)]
fn check(s: &str, t: &str, exp: &str) {
    assert_eq!(exp.to_string(), min_window(s.to_string(), t.to_string()))
}

#[test]
fn test1() {
    check("ADOBECODEBANC", "ABC", "BANC")
}

#[test]
fn test2(){
    check("a", "a", "a")
}

#[test]
fn test3(){
    check("a", "aa", "")
}

struct MinWindow {
    counts: [i32; 128],
    need: i32,
}

impl MinWindow {
    fn new() -> Self {
        Self {
            counts: [0; 128],
            need: 0,
        }
    }
    fn char(&mut self, c: char) -> MinWindowChar {
        let pos = c as u8 as usize;
        MinWindowChar {
            val: self.counts[pos],
            mw: self,
            pos,
        }
    }
}

struct MinWindowChar<'a> {
    mw: &'a mut MinWindow,
    val: i32,
    pos: usize,
}

impl Deref for MinWindowChar<'_> {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl DerefMut for MinWindowChar<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}

impl Drop for MinWindowChar<'_> {
    fn drop(&mut self) {
        let old = &mut self.mw.counts[self.pos];
        if (*old >= 0) != (self.val >= 0) {
            self.mw.need += 2 * (self.val < 0) as i32 - 1;
        }
        *old = self.val
    }
}
