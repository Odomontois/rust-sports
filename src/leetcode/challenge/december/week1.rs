use crate::leetcode::data::{List, Tree};
use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;

struct RandomChoose<R: Rng> { items: Vec<i32>, rng: R }

type Solution = RandomChoose<ThreadRng>;

impl Solution {
    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(mut head: List) -> Self {
        let mut items = Vec::new();
        while let Some(h) = head {
            items.push(h.val);
            head = h.next;
        }
        RandomChoose { items, rng: thread_rng() }
    }
}

impl<R: Rng> RandomChoose<R> {
    /** Returns a random node's value. */
    fn get_random(&mut self) -> i32 {
        let i = self.rng.gen_range(0, self.items.len());
        self.items[i]
    }
}

use std::mem::swap;

// pub type Tree = Option<Rc<RefCell<TreeNode>>>;



pub fn increasing_bst(root: Tree) -> Tree {
    let mut cur = root.clone();
    while let Some(rc) = cur {
        let rm = &mut *rc.borrow_mut();
        while let Some(lc) = rm.left.clone() {
            let lm = &mut *lc.borrow_mut();
            // [a - x - b] - y - c
            swap(&mut lm.right, &mut lm.left);
            // [b - x - a] - y - c
            swap(&mut lm.val, &mut rm.val);
            // [b - y - a] - x - c
            swap(&mut lm.right, &mut rm.right);
            // [b - y - c] - x - a
            swap(&mut rm.left, &mut rm.right);
            // a - x - [b - y - c]
        }
        cur = rm.right.clone();
    }
    root
}
