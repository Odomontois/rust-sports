use std::{cell::RefCell, mem::swap, rc::Rc, vec};

use crate::leetcode::data::{Tree, TreeNode};

pub fn recover_tree(root: &mut Tree) {
    // let mut stack = vec![];
    // iter_recover(root.clone(), &mut stack);
    todo!()
}

// pub fn iter_recover<'a>(root: Tree, stack: &mut Vec<(bool, i32, Rc<RefCell<TreeNode>>)>) -> Option<&'a mut i32> {
//     let r = root?;
//     let mut node = r.borrow_mut();
//     for (left, v, parent) in stack.iter() {
//         if (node.val < *v) != *left {
//             let mut pn = parent.borrow_mut();
//             swap(&mut (pn.val), &mut (node.val))
//         }
//     }
//     stack.push((true, node.val, r.clone()));
//     iter_recover(node.left.clone(), stack);
//     stack.pop();

//     stack.push((false, node.val, r.clone()));

//     None
// }
