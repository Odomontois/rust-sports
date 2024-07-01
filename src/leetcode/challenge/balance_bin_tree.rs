use data::leetcode::*;
use std::{cell::RefCell, mem::swap, rc::Rc};

pub type Tree = Option<Rc<RefCell<TreeNode>>>;

pub fn balance_bst(root: Tree) -> Tree {
    balance_node(&root, size(&root));
    root
}

fn balance_node(tree: &Tree, size: usize) {
    let Some(node) = tree else { return };
    bring_up(tree, size / 2);
    let node = node.borrow();
    balance_node(&node.left, size / 2);
    balance_node(&node.right, (size - 1) / 2);
}

fn bring_up(tree: &Tree, ix: usize) -> Option<usize> {
    let Some(node) = tree else { return Some(0) };
    let node = &mut *node.borrow_mut();
    let Some(ls) = bring_up(&node.left, ix) else {
        rotate_right(node);
        return None;
    };
    if ix == ls {
        return None;
    }
    let Some(rs) = bring_up(&node.right, ix - ls - 1) else {
        rotate_left(node);
        return None;
    };
    Some(ls + rs + 1)
}

fn size(tree: &Tree) -> usize {
    let Some(node) = tree else { return 0 };
    let node = &*node.borrow();
    size(&node.left) + size(&node.right) + 1
}

fn rotate_right(tree: &mut TreeNode) {
    swap(&mut tree.left, &mut tree.right);
    let Some(left) = &tree.right else { return };
    let mut left = left.borrow_mut();
    swap(&mut left.val, &mut tree.val);
    swap(&mut left.right, &mut tree.left);
    swap(&mut left.left, &mut tree.left);
}

fn rotate_left(tree: &mut TreeNode) {
    swap(&mut tree.left, &mut tree.right);
    let Some(right) = &tree.left else { return };
    let mut right = right.borrow_mut();
    swap(&mut right.val, &mut tree.val);
    swap(&mut right.left, &mut tree.right);
    swap(&mut right.right, &mut tree.right);
}

#[test]
fn test1() {
    for t in [
        (1, None, (2, None, (3, None, 4))).tree(),
        (1, None, (2, None, (3, None, (4, None, (5, None, (6, None, 7)))))).tree(),
    ] {
        println!("{}", Displayed(&balance_bst(t)));
    }
}
