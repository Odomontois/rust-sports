use std::iter::FromIterator;

use crate::leetcode::challenge::november::data::Tree;

#[allow(dead_code)]
pub fn rob(root: Tree) -> i32 {
    rob_iter(&root).0
}

fn rob_iter(tree: &Tree) -> (i32, i32) {
    if let Some(noderef) = tree {
        let r = noderef.borrow();
        let (lall, ldown) = rob_iter(&r.left);
        let (rall, rdown) = rob_iter(&r.right);
        let down = lall + rall;
        let all = (r.val + ldown + rdown).max(down);
        (all, down)
    } else {
        (0, 0)
    }
}


#[test]
fn test_calculate() {
    fn check(s: &str, res: i32) { assert_eq!(calculate(s.to_string()), res) }
    check("3+2*2", 7);
    check("  3/2   ", 1);
    check("3+5    /   2", 5);
    println!("{}", std::mem::size_of::<Calculate>());
    println!("{}", std::mem::size_of::<(Option<AddOp>, Option<MulOp>)>());
}

#[allow(dead_code)]
pub fn calculate(s: String) -> i32 {
    s.chars().collect::<Calculate>().result()
}

enum AddOp { Plus, Minus }

enum MulOp { Times, Div }

struct Calculate { add: i32, add_op: Option<AddOp>, mul: i32, mul_op: Option<MulOp>, cur: i32 }

impl Calculate {
    fn result(self) -> i32 { self.next_add() }

    fn next_add(&self) -> i32 {
        let m = self.next_mul();
        match self.add_op {
            Some(AddOp::Plus) => self.add + m,
            Some(AddOp::Minus) => self.add - m,
            None => m
        }
    }
    fn next_mul(&self) -> i32 {
        match self.mul_op {
            Some(MulOp::Times) => self.mul * self.cur,
            Some(MulOp::Div) => self.mul / self.cur,
            None => self.cur
        }
    }
    fn feed_mul(&mut self, op: MulOp) {
        self.mul = self.next_mul();
        self.mul_op = Some(op);
        self.cur = 0
    }
    fn feed_add(&mut self, op: AddOp) {
        self.add = self.next_add();
        self.add_op = Some(op);
        self.mul_op = None;
        self.cur = 0;
    }
    fn feed(&mut self, c: char) {
        match (c, c.to_digit(10)) {
            (_, Some(d)) => self.cur = self.cur * 10 + d as i32,
            ('*', _) => self.feed_mul(MulOp::Times),
            ('/', _) => self.feed_mul(MulOp::Div),
            ('+', _) => self.feed_add(AddOp::Plus),
            ('-', _) => self.feed_add(AddOp::Minus),
            _ => {}
        }
    }
}

impl FromIterator<char> for Calculate {
    fn from_iter<T: IntoIterator<Item=char>>(iter: T) -> Self {
        let mut calc = Calculate { add: 0, add_op: None, mul: 1, mul_op: None, cur: 0 };
        for c in iter { calc.feed(c) }
        calc
    }
}
