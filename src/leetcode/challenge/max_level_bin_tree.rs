use data::leetcode::TreeNode;
use std::{cell::RefCell, rc::Rc};

type Tree = Option<Rc<RefCell<TreeNode>>>;

pub fn max_level_sum(root: Tree) -> i32 {
    let mut go = Go::default();
    go.go(&root, 0);
    let levels = go.levels.iter().rev().enumerate();
    levels.max_by_key(|p| p.1).map(|p| p.0 as i32).unwrap_or(0)
}

#[derive(Default)]
struct Go {
    levels: Vec<i32>,
}

impl Go {
    fn go(&mut self, tree: &Tree, level: usize) {
        let node = if let Some(node) = tree { node.borrow() } else { return };
        if self.levels.len() <= level {
            self.levels.push(0);
        }
        self.levels[level] += node.val;
        self.go(&node.left, level + 1);
        self.go(&node.right, level + 1);
    }
}
