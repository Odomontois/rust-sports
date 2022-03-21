pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut solver = Solver::default();
    solver.init();
    for row in 0..9 {
        for col in 0..9 {
            let c = board[row as usize][col as usize];
            if c == '.' {
                continue;
            }
            let val = (c as u8 - '1' as u8) as usize;
            solver.use_variant(variant(row, col, val))
        }
    }
    if !solver.solve() {
        panic!("No solution")
    }
    for var in solver.solution {
        let (row, col, val) = unvariant(var);
        board[row as usize][col as usize] = (val as u8 + '1' as u8) as char;
    }
}

use std::ops::{Deref, DerefMut};

fn restriction(kind: usize, x: usize, y: usize) -> usize {
    kind * 81 + x * 9 + y
}

fn variant(row: usize, col: usize, value: usize) -> usize {
    row * 81 + col * 9 + value
}
fn unvariant(var: usize) -> (usize, usize, usize) {
    (var / 81, var / 9 % 9, var % 9)
}
fn restrs(var: usize) -> Vec<usize> {
    let (row, col, val) = unvariant(var);
    let block = row / 3 * 3 + col / 3;
    vec![
        restriction(0, row, val),
        restriction(1, col, val),
        restriction(2, block, val),
        restriction(3, row, col),
    ]
}

#[derive(Default, Clone, Copy)]
struct LinkNode {
    data: usize,
    prev: usize,
    next: usize,
}

impl LinkNode {
    fn of(data: usize) -> Self {
        Self { data, prev: 0, next: 0 }
    }
    fn loopy(idx: usize) -> Self {
        Self {
            data: 0,
            prev: idx,
            next: idx,
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Rel {
    restriction: LinkNode,
    variant: LinkNode,
}

#[derive(Default)]
struct Solver {
    rels: Vec<Rel>,
    rqueue: Vec<LinkNode>,
    rcounts: Vec<usize>,
    solution: Vec<usize>,
}

trait Accessor: Copy {
    fn access<'a>(&self, src: &'a Solver, idx: usize) -> &'a LinkNode;
    fn access_mut<'a>(&self, src: &'a mut Solver, idx: usize) -> &'a mut LinkNode;
}

struct Link<'a, A: Accessor> {
    root: &'a mut Solver,
    idx: usize,
    acc: A,
}

impl<'a, A: Accessor> Link<'a, A> {
    fn next_link(&mut self) -> Link<A> {
        let idx = self.next;
        Link {
            root: self.root,
            idx,
            acc: self.acc,
        }
    }

    fn prev_link(&mut self) -> Link<A> {
        let idx = self.prev;
        Link {
            root: self.root,
            acc: self.acc,
            idx,
        }
    }

    fn remove(&mut self) {
        self.acc.access_mut(self.root, self.next).prev = self.prev;
        self.acc.access_mut(self.root, self.prev).next = self.next;
    }

    fn restore(&mut self) {
        self.acc.access_mut(self.root, self.next).prev = self.idx;
        self.acc.access_mut(self.root, self.prev).next = self.idx;
    }
}

impl<'a, A: Accessor> IntoIterator for Link<'a, A> {
    type Item = &'a mut LinkNode;

    type IntoIter = LinkIter<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        LinkIter {
            idx: None,
            idx_back: None,
            acc: self.acc,
            root: self.root,
            start: self.idx,
        }
    }
}

impl<'a, A: Accessor> Deref for Link<'a, A> {
    type Target = LinkNode;

    fn deref(&self) -> &Self::Target {
        self.acc.access(self.root, self.idx)
    }
}

impl<'a, A: Accessor> DerefMut for Link<'a, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.acc.access_mut(self.root, self.idx)
    }
}

struct LinkIter<'a, A: Accessor> {
    root: &'a mut Solver,
    start: usize,
    idx: Option<usize>,
    idx_back: Option<usize>,
    acc: A,
}

impl<'a, A: Accessor> Iterator for LinkIter<'a, A> {
    type Item = &'a mut LinkNode;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.acc.access(self.root, self.idx.unwrap_or(self.start)).next;
        self.idx = Some(next);
        if next == self.start {
            None
        } else {
            let root: &'a mut Solver = unsafe { &mut *(self.root as *mut _) };
            Some(self.acc.access_mut(root, next))
        }
    }
}

impl<'a, A: Accessor> DoubleEndedIterator for LinkIter<'a, A> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let prev = self.acc.access(self.root, self.idx_back.unwrap_or(self.start)).prev;
        self.idx_back = Some(prev);
        if prev == self.start {
            None
        } else {
            let root: &'a mut Solver = unsafe { &mut *(self.root as *mut _) };
            Some(self.acc.access_mut(root, prev))
        }
    }
}

impl Solver {
    const RESTR_COUNT: usize = 4 * 9 * 9;
    const VAR_COUNT: usize = 9 * 9 * 9;
    fn init(&mut self) {
        self.rqueue = vec![LinkNode::default(); 10 + Self::RESTR_COUNT];
        self.rcounts = vec![9; Self::RESTR_COUNT];
        for rest in 0..Self::RESTR_COUNT {
            let rel = Rel {
                restriction: LinkNode::of(rest),
                variant: LinkNode::loopy(self.rels.len()),
            };
            self.rels.push(rel);
            self.rqueue[10 + rest].data = rest;
            self.rqueue_insert(rest);
        }
        for var in 0..Self::VAR_COUNT {
            let rel = Rel {
                variant: LinkNode::of(var),
                restriction: LinkNode::loopy(self.rels.len()),
            };
            self.rels.push(rel);
        }
        for var in 0..Self::VAR_COUNT {
            let variant = LinkNode::of(var);
            for restr in restrs(var) {
                let restriction = LinkNode::of(restr);
                self.register(Rel { variant, restriction })
            }
        }
    }

    fn register(&mut self, mut rel: Rel) {
        let i = self.rels.len();
        let ridx = rel.restriction.data;
        let vnext = &mut self.rels[ridx].variant.next;
        rel.variant.next = *vnext;
        rel.variant.prev = ridx;
        *vnext = i;
        self.rels[rel.variant.next].variant.prev = i;
        let vidx = rel.variant.data + Self::RESTR_COUNT;
        let rnext = &mut self.rels[vidx].restriction.next;
        rel.restriction.next = *rnext;
        rel.restriction.prev = vidx;
        *rnext = i;
        self.rels[rel.restriction.next].restriction.prev = i;
        self.rels.push(rel);
    }

    fn rqueue_insert(&mut self, r: usize) {
        let idx = 10 + r;
        let count = self.rcounts[r];
        let next = self.rqueue[count].next;

        self.rqueue[next].prev = idx;
        self.rqueue[count].next = idx;
        self.rqueue[idx].prev = count;
        self.rqueue[idx].next = next;
    }

    fn rqueue_remove(&mut self, rest: usize) {
        let idx = 10 + rest;
        let ln = self.rqueue[idx];

        self.rqueue[ln.next].prev = ln.prev;
        self.rqueue[ln.prev].next = ln.next;
    }

    fn restr_resize(&mut self, rest: usize, f: impl Fn(&mut usize)) {
        self.rqueue_remove(rest);
        f(&mut self.rcounts[rest]);
        self.rqueue_insert(rest);
    }

    fn with_variant(&mut self, v: usize, back: bool, mut f: impl FnMut(&mut Self, usize, Rel)) {
        let start = v + Self::RESTR_COUNT;
        let mut cur = start;

        loop {
            cur = if back {
                self.rels[cur].restriction.prev
            } else {
                self.rels[cur].restriction.next
            };
            if cur == start {
                return;
            }
            let rel = self.rels[cur];
            f(self, cur, rel);
        }
    }

    fn with_restriction(&mut self, r: usize, back: bool, mut f: impl FnMut(&mut Self, usize, Rel)) {
        let start = r;
        let mut cur = start;

        loop {
            cur = if back {
                self.rels[cur].variant.prev
            } else {
                self.rels[cur].variant.next
            };
            if cur == start {
                return;
            }
            let rel = self.rels[cur];
            f(self, cur, rel);
        }
    }

    fn get_variants(&mut self, r: usize) -> Vec<usize> {
        let mut res = vec![];
        self.with_restriction(r, false, |_, _, rel| {
            res.push(rel.variant.data);
        });
        res
    }

    fn drop_variant(&mut self, v: usize) {
        self.with_variant(v, false, |me, _, rel| {
            me.rels[rel.variant.prev].variant.next = rel.variant.next;
            me.rels[rel.variant.next].variant.prev = rel.variant.prev;
            let rest = rel.restriction.data;
            me.restr_resize(rest, |v| *v -= 1);
        });
        self.solution.push(v);
    }

    fn restore_variant(&mut self, v: usize) {
        self.with_variant(v, true, |me, cur, rel| {
            let rest = rel.restriction.data;
            me.restr_resize(rest, |v| *v += 1);
            me.rels[rel.variant.prev].variant.next = cur;
            me.rels[rel.variant.next].variant.prev = cur;
        })
    }

    fn use_variant(&mut self, v: usize) {
        self.with_variant(v, false, |me, _, rel| {
            me.rels[rel.variant.prev].variant.next = rel.variant.next;
            me.rels[rel.variant.next].variant.prev = rel.variant.prev;
            me.close_restriction(rel.restriction.data)
        });
        self.solution.push(v);
    }

    fn unuse_variant(&mut self, v: usize) {
        self.with_variant(v, true, |me, cur, rel| {
            me.open_restriction(rel.restriction.data);
            me.rels[rel.variant.prev].variant.next = cur;
            me.rels[rel.variant.next].variant.prev = cur;
        });
        self.solution.pop();
    }

    fn close_restriction(&mut self, r: usize) {
        self.rqueue_remove(r);
        self.with_restriction(r, false, |me, _, rel| {
            me.rels[rel.restriction.next].restriction.prev = rel.restriction.prev;
            me.rels[rel.restriction.prev].restriction.next = rel.restriction.next;
            me.drop_variant(rel.variant.data);
        })
    }

    fn open_restriction(&mut self, r: usize) {
        self.with_restriction(r, true, |me, idx, rel| {
            me.restore_variant(rel.variant.data);
            me.rels[rel.restriction.next].restriction.prev = idx;
            me.rels[rel.restriction.prev].restriction.next = idx;
        });
        self.rqueue_insert(r);
    }

    fn smallest(&self) -> Option<(usize, usize)> {
        for (c, q) in self.rqueue[0..10].iter().enumerate() {
            if q.next != 0 {
                let r = self.rqueue[q.next].data;
                return Some((c, r));
            }
        }
        None
    }

    fn solve(&mut self) -> bool {
        let (c, restr) = if let Some(p) = self.smallest() { p } else { return true };

        if c == 0 {
            return false;
        }

        for v in self.get_variants(restr) {
            self.use_variant(v);
            if self.solve() {
                return true;
            }
            self.unuse_variant(v);
        }

        false
    }
}

#[derive(Clone, Copy)]
struct RestrAcc;
impl Accessor for RestrAcc {
    fn access<'a>(&self, src: &'a Solver, idx: usize) -> &'a LinkNode {
        &src.rels[idx].restriction
    }

    fn access_mut<'a>(&self, src: &'a mut Solver, idx: usize) -> &'a mut LinkNode {
        &mut src.rels[idx].restriction
    }
}
#[derive(Clone, Copy)]
struct VarAcc;
impl Accessor for VarAcc {
    fn access<'a>(&self, src: &'a Solver, idx: usize) -> &'a LinkNode {
        &src.rels[idx].variant
    }

    fn access_mut<'a>(&self, src: &'a mut Solver, idx: usize) -> &'a mut LinkNode {
        &mut src.rels[idx].variant
    }
}
#[derive(Clone, Copy)]
struct RQAcc;
impl Accessor for RQAcc {
    fn access<'a>(&self, src: &'a Solver, idx: usize) -> &'a LinkNode {
        &src.rqueue[idx]
    }

    fn access_mut<'a>(&self, src: &'a mut Solver, idx: usize) -> &'a mut LinkNode {
        &mut src.rqueue[idx]
    }
}

#[cfg(test)]
fn check(s: &[&str], exp: &[&str]) {
    let mut vec = s.iter().map(|s| s.chars().collect()).collect();
    solve_sudoku(&mut vec);
    let exp: Vec<Vec<char>> = exp.iter().map(|s| s.chars().collect()).collect();
    assert_eq!(exp, vec)
}

#[test]
fn test1() {
    check(
        &[
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ],
        &[
            "534678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "287419635",
            "345286179",
        ],
    )
}

#[test]
fn test2() {
    check(
        &[
            "..9748...",
            "7........",
            ".2.1.9...",
            "..7...24.",
            ".64.1.59.",
            ".98...3..",
            "...8.3.2.",
            "........6",
            "...2759..",
        ],
        &[
            "519748632",
            "783652419",
            "426139875",
            "357986241",
            "264317598",
            "198524367",
            "975863124",
            "832491756",
            "641275983",
        ],
    )
}
