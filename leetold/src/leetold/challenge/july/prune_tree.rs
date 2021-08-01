use data::leetcode::{Tree, TreeNode};
use std::{cell::RefCell, rc::Rc};

pub fn prune_tree(root: Tree) -> Tree {
    let t = root?;
    let x = &*t.borrow();
    match (prune_tree(x.left.clone()), prune_tree(x.right.clone()), x.val) {
        (None, None, 0) => None,
        (left, right, val) => Some(Rc::new(RefCell::new(TreeNode { val, left, right }))),
    }
}
