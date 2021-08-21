pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut solver = Solver::default();
    solver.init();
    for row in 0..9 {
        for col in 0..9 {
            let c = board[row as usize][col as usize];
            if c == '.' {
                continue;
            }
            let val = c as u8 - '0' as u8;
            solver.use_variant(Variant { col, row, val })
        }
    }
    if !solver.solve() {
        panic!("No solution")
    }
    for Variant { row, col, val } in solver.solution{
        board[row as usize][col as usize] = (val + '0' as u8) as char;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum RestrictKind {
    Row,
    Col,
    Block,
    Cell,
}
use RestrictKind::*;

trait Idx {
    fn by_idx(i: usize) -> Self;
    fn idx(&self) -> usize;
}

impl Default for RestrictKind {
    fn default() -> Self {
        Row
    }
}

impl Idx for RestrictKind {
    fn idx(&self) -> usize {
        match self {
            Row => 0,
            Col => 1,
            Block => 2,
            Cell => 3,
        }
    }

    fn by_idx(i: usize) -> Self {
        match i {
            0 => Row,
            1 => Col,
            2 => Block,
            _ => Cell,
        }
    }
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Restriction {
    kind: RestrictKind,
    x: u8,
    y: u8,
}

impl Idx for Restriction {
    fn idx(&self) -> usize {
        self.kind.idx() * 81 + self.x as usize * 9 + self.y as usize
    }

    fn by_idx(i: usize) -> Self {
        Self {
            kind: RestrictKind::by_idx(i / 81),
            x: (i / 9 % 9) as u8,
            y: (i % 9) as u8,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Variant {
    row: u8,
    col: u8,
    val: u8,
}

impl Idx for Variant {
    fn idx(&self) -> usize {
        self.row as usize * 81 + self.col as usize * 9 + self.val as usize
    }

    fn by_idx(i: usize) -> Self {
        Self {
            row: (i / 81) as u8,
            col: (i / 9 % 9) as u8,
            val: (i % 9) as u8,
        }
    }
}

impl Variant {
    fn block(&self) -> u8 {
        self.row / 3 * 3 + self.col / 3
    }
    fn restrs(&self) -> Vec<Restriction> {
        vec![
            Restriction {
                kind: Row,
                x: self.row,
                y: self.val,
            },
            Restriction {
                kind: Col,
                x: self.col,
                y: self.val,
            },
            Restriction {
                kind: Block,
                x: self.block(),
                y: self.val,
            },
            Restriction {
                kind: Cell,
                x: self.row,
                y: self.col,
            },
        ]
    }
}

#[derive(Default, Clone, Copy)]
struct LinkNode<A> {
    data: A,
    prev: usize,
    next: usize,
}

impl<A> LinkNode<A> {
    fn of(data: A) -> Self {
        Self { data, prev: 0, next: 0 }
    }
}

#[derive(Default, Clone, Copy)]
struct Rel {
    restriction: LinkNode<Restriction>,
    variant: LinkNode<Variant>,
}

#[derive(Default)]
struct Solver {
    rels: Vec<Rel>,
    rqueue: Vec<LinkNode<Restriction>>,
    rcounts: Vec<usize>,
    solution: Vec<Variant>,
}

impl Solver {
    const RESTR_COUNT: usize = 4 * 9 * 9;
    const VAR_COUNT: usize = 9 * 9 * 9;
    fn init(&mut self) {
        self.rqueue = vec![LinkNode::default(); 9 + Self::RESTR_COUNT];
        for rest in (0..Self::RESTR_COUNT).map(Restriction::by_idx) {
            let ln = LinkNode::of(rest);
            let rel = Rel {
                restriction: ln,
                ..Default::default()
            };
            self.rels.push(rel);
            self.rqueue_insert(9, rest);
        }
        for var in (0..Self::VAR_COUNT).map(Variant::by_idx) {
            let ln = LinkNode::of(var);
            let rel = Rel {
                variant: ln,
                ..Default::default()
            };
            self.rels.push(rel);
        }
        for var in (0..Self::VAR_COUNT).map(Variant::by_idx) {
            let variant = LinkNode::of(var);
            for restr in var.restrs() {
                let restriction = LinkNode::of(restr);
                self.register(Rel { variant, restriction })
            }
        }
    }

    fn register(&mut self, mut rel: Rel) {
        let i = self.rels.len();
        let ridx = rel.restriction.data.idx();
        let vnext = &mut self.rels[ridx].variant.next;
        rel.variant.next = *vnext;
        rel.variant.prev = ridx;
        *vnext = i;
        self.rels[rel.variant.next].variant.prev = i;
        let vidx = rel.variant.data.idx() + Self::RESTR_COUNT;
        let rnext = &mut self.rels[vidx].restriction.next;
        rel.restriction.next = *rnext;
        rel.restriction.prev = vidx;
        *rnext = i;
        self.rels[rel.restriction.next].restriction.prev = i;
        self.rels.push(rel);
    }

    fn rqueue_insert(&mut self, count: usize, r: Restriction) {
        let idx = 9 + r.idx();
        let next = self.rqueue[count].next;
        self.rqueue[next].prev = idx;
        self.rqueue[count].next = idx;
    }

    fn rqueue_remove(&mut self, rest: Restriction) {
        let ln = self.rqueue[rest.idx() + 9];
        self.rqueue[ln.next].prev = ln.prev;
        self.rqueue[ln.prev].next = ln.next;
    }

    fn use_variant(&mut self, v: Variant) {
        self.with_variant(v, |me, _, rel| {
            me.rels[rel.variant.prev].variant.next = rel.variant.next;
            me.rels[rel.variant.next].variant.prev = rel.variant.prev;
            let rest = rel.restriction.data;
            me.restr_resize(rest, |v| *v -= 1);
        });
        self.solution.push(v);
    }

    fn restr_resize(&mut self, rest: Restriction, f: impl Fn(&mut usize)) {
        self.rqueue_remove(rest);
        f(&mut self.rcounts[rest.idx()]);
        self.rqueue_insert(self.rcounts[rest.idx()], rest);
    }

    fn unuse_variant(&mut self) {
        let v = if let Some(p) = self.solution.pop() { p } else { return };
        self.with_variant(v, |me, cur, rel| {
            me.rels[rel.variant.prev].variant.next = cur;
            me.rels[rel.variant.next].variant.prev = cur;
            let rest = rel.restriction.data;
            me.restr_resize(rest, |v| *v += 1);
        })
    }

    fn with_variant(&mut self, v: Variant, mut f: impl FnMut(&mut Self, usize, Rel)) {
        let mut cur = self.rels[v.idx() + Self::RESTR_COUNT].restriction.next;
        while cur != 0 {
            let rel = self.rels[cur];
            f(self, cur, rel);
            cur = self.rels[cur].restriction.next;
        }
    }

    fn with_restriction(&mut self, r: Restriction, mut f: impl FnMut(&mut Self, usize, Rel)) {
        let mut cur = self.rels[r.idx()].variant.next;
        while cur != 0 {
            let rel = self.rels[cur];
            f(self, cur, rel);
            cur = self.rels[cur].variant.next;
        }
    }

    fn consume_variants(&mut self, r: Restriction) -> Vec<Variant> {
        let mut res = vec![];
        self.with_restriction(r, |me, _, rel| {
            me.rels[rel.restriction.prev].restriction.next = rel.restriction.next;
            me.rels[rel.restriction.next].restriction.prev = rel.restriction.prev;
            res.push(rel.variant.data);
        });
        res
    }

    fn restore_variants(&mut self, r: Restriction) {
        self.with_restriction(r, |me, cur, rel| {
            me.rels[rel.restriction.prev].restriction.next = cur;
            me.rels[rel.restriction.next].restriction.prev = cur;
        })
    }

    fn smallest(&self) -> Restriction {
        let q = self.rqueue.iter().filter(|r| r.next != 0).next();
        q.copied().unwrap_or_default().data
    }

    fn solve(&mut self) -> bool {
        let restriction = self.smallest();
        if self.rcounts[restriction.idx()] == 0 {
            return false;
        }
        for v in self.consume_variants(restriction) {
            self.use_variant(v);
            if self.solve() {
                return true;
            }
            self.unuse_variant();
        }
        self.restore_variants(restriction);

        false
    }
}
