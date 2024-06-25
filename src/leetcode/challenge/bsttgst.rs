use data::leetcode::*;
use std::{cell::RefCell, rc::Rc};
type Tree = Option<Rc<RefCell<TreeNode>>>;
pub fn bst_to_gst(root: Tree) -> Tree {
    walk(&root, 0);
    root
}

pub fn walk(tree: &Tree, add: i32) -> i32 {
    let Some(tree) = tree else {
        return add;
    };
    let mut tree = tree.borrow_mut();
    let add = walk(&mut tree.right, add) + tree.val;
    tree.val = add;
    walk(&mut tree.left, add)
}

#[test]
fn check1() {
    let tree = bst_to_gst((4, (1, 0, (2, None, 3)), (6, 5, (7, None, 8))).tree());
    println!("{}", Displayed(&tree));
}
