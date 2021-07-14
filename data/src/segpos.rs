use std::{mem::size_of, ops::Range};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct SegPos {
    pub p: usize,
    pub from: usize,
    pub until: usize,
}

const USIZE_LEN: u32 = 8 * size_of::<usize>() as u32;
impl SegPos {
    pub fn start(until: usize) -> Self {
        Self { p: 0, from: 0, until }
    }

    pub fn elem(&self) -> bool {
        self.until - self.from == 1
    }

    pub fn contains(&self, i: usize) -> bool {
        (self.from..self.until).contains(&i)
    }

    pub fn subs(&self) -> (Self, Self) {
        if self.from == self.until {
            return (*self, *self);
        }
        let d = self.until - self.from;
        let k = 1 << (USIZE_LEN - d.leading_zeros() - 2);
        let m = if 3 * k >= d { self.until - k } else { self.from + k * 2 };
        let l = Self {
            p: self.p * 2 + 1,
            from: self.from,
            until: m,
        };
        let r = Self {
            p: self.p * 2 + 2,
            from: m,
            until: self.until,
        };
        (l, r)
    }

    pub fn intersects(&self, range: &Range<usize>) -> bool {
        self.from < range.end && self.until > range.start
    }

    pub fn inside(&self, range: &Range<usize>) -> bool {
        self.from >= range.start && self.until <= range.end
    }
}
