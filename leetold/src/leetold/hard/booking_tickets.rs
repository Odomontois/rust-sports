// https://leetcode.com/problems/booking-concert-tickets-in-groups/

use std::ops::{Add, SubAssign};

#[derive(Debug, Clone, Copy, Default)]
struct Free {
    total: u64,
    together: u64,
}

impl Add for Free {
    type Output = Free;

    fn add(self, rhs: Self) -> Free {
        Free {
            total: self.total + rhs.total,
            together: self.together.max(rhs.together),
        }
    }
}

impl SubAssign<u64> for Free {
    fn sub_assign(&mut self, rhs: u64) {
        self.total -= rhs;
        self.together -= rhs;
    }
}

struct BookMyShow {
    fen: Vec<Free>,
    size: usize,
    cap: u64,
}

struct Pos {
    p: usize,
    start: usize,
    end: usize,
}

impl Pos {
    fn next(&self) -> Option<(Pos, Pos)> {
        let &Pos { p, start, end } = self;
        (end - start > 1).then(|| {
            let m = (end + start + 1) / 2;
            (
                Pos {
                    p: 2 * p + 1,
                    start,
                    end: m,
                },
                Pos {
                    p: 2 * p + 2,
                    start: m,
                    end,
                },
            )
        })
    }
}
impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        let size = n as usize;
        let fen = vec![Free { total: 0, together: 0 }; size * 4];
        let cap = m as u64;
        let mut res = Self { fen, size, cap };
        res.init(res.start());
        res
    }

    fn start(&self) -> Pos {
        Pos {
            p: 0,
            start: 0,
            end: self.size,
        }
    }

    fn update(&mut self, p: Pos) {
        if let Some((l, r)) = p.next() {
            self.fen[p.p] = self.fen[l.p] + self.fen[r.p]
        }
    }

    fn init(&mut self, pos: Pos) {
        if let Some((l, r)) = pos.next() {
            self.init(l);
            self.init(r);
            self.update(pos);
        } else {
            self.fen[pos.p].together = self.cap;
            self.fen[pos.p].total = self.cap;
        }
    }

    fn range_calc(&self, pos: Pos, row: usize) -> Free {
        if pos.start > row {
            return Free::default();
        }
        if pos.end <= row + 1 {
            return self.fen[pos.p];
        }
        if let Some((l, r)) = pos.next() {
            self.range_calc(l, row) + self.range_calc(r, row)
        } else {
            self.fen[pos.p]
        }
    }

    fn gather_rec(&mut self, pos: Pos, k: u64, row: usize) -> Option<(usize, u64)> {
        if pos.start > row || self.fen[pos.p].together < k {
            return None;
        }
        if let Some((l, r)) = pos.next() {
            let res = self.gather_rec(l, k, row).or_else(|| self.gather_rec(r, k, row));
            self.update(pos);
            res
        } else {
            let was = self.cap - self.fen[pos.p].total;
            self.fen[pos.p] -= k;
            Some((pos.start, was))
        }
    }

    fn scatter_rec(&mut self, pos: Pos, k: u64) -> u64 {
        if self.fen[pos.p].total == 0 {
            return 0;
        }
        if let Some((l, r)) = pos.next() {
            let x = self.scatter_rec(l, k);
            let res = if x == k { k } else { x + self.scatter_rec(r, k - x) };
            self.update(pos);
            res
        } else {
            let res = self.fen[pos.p].total.min(k);
            self.fen[pos.p] -= res;
            return res;
        }
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        if let Some((row, seat)) = self.gather_rec(self.start(), k as u64, max_row as usize) {
            vec![row as i32, seat as i32]
        } else {
            vec![]
        }
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        self.range_calc(self.start(), max_row as usize).total >= k as u64 && {
            self.scatter_rec(self.start(), k as u64) == k as u64
        }
    }
}

#[test]
fn example1() {
    let mut book = BookMyShow::new(2, 5);
    assert_eq!(book.gather(4, 0), vec![0, 0],);
    assert_eq!(book.gather(2, 0), vec![]);
    assert!(book.scatter(5, 1));
    assert!(!book.scatter(5, 1));
}
