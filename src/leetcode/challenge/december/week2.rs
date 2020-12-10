use crate::leetcode::data::{Tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
    let mut dd = vec![0; 60];
    for t in time { dd[(t % 60) as usize] += 1 };
    (dd[0] * (dd[0] - 1) / 2) + (dd[30] * (dd[30] - 1) / 2) + (1..30).map(|i| dd[i] * dd[60 - i]).sum::<i32>()
}

#[test]
fn test_num_pairs() {
    println!("{}", num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]));
}


struct BSTIterator { stack: Vec<StackItem> }

enum StackItem {
    Val(i32),
    Node(Rc<RefCell<TreeNode>>),
}

use StackItem::*;

impl BSTIterator {
    fn new(root: Tree) -> Self { BSTIterator { stack: root.into_iter().map(Node).collect() } }

    fn next(&mut self) -> i32 {
        while let Some(i) = self.stack.pop() {
            match i {
                Val(x) => return x,
                Node(nr) => {
                    let n = nr.borrow();
                    for r in n.right.clone() { self.stack.push(Node(r)) }
                    self.stack.push(Val(n.val));
                    for l in n.left.clone() { self.stack.push(Node(l)) }
                }
            }
        }
        -1000
    }

    fn has_next(&self) -> bool { !self.stack.is_empty() }
}

pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    arr.windows(2).fold(0, |d, w| if d == -1 { -1 } else if w[0] > w[1] && d > 0 { 2 } else if w[0] < w[1] && d < 2 { 1 } else { -1 }) == 2
}