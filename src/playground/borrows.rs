use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Rc<RefCell<TreeNode>>>;

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
