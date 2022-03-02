use std::{cell::RefCell, iter::from_fn, rc::Rc};

use data::leetcode::{Tree, TreeNode};

// https://leetcode.com/problems/all-elements-in-two-binary-search-trees/

pub fn get_all_elements(root1: Tree, root2: Tree) -> Vec<i32> {
    let mut it1 = TreeIter::new(root1).peekable();
    let mut it2 = TreeIter::new(root2).peekable();
    let gen = || {
        if it1.peek().into_iter().any(|x| it2.peek().into_iter().all(|y| x < y)) {
            it1.next()
        } else {
            it2.next()
        }
    };
    from_fn(gen).collect()
}

struct TreeIter {
    stack: Vec<Rc<RefCell<TreeNode>>>,
    cur: Tree,
}

impl TreeIter {
    fn new(cur: Tree) -> Self {
        Self { cur, stack: vec![] }
    }
}

impl Iterator for TreeIter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let clone_left = |n: &Rc<RefCell<TreeNode>>| n.borrow().left.clone();
        let bot = if let Some(mut bot) = self.cur.take() {
            while let Some(left) = clone_left(&bot) {
                self.stack.push(bot);
                bot = left;
            }
            bot
        } else {
            self.stack.pop()?
        };
        let bot = bot.borrow();
        self.cur = bot.right.clone();
        return Some(bot.val);
    }
}
