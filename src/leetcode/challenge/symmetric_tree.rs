use data::leetcode::TreeNode;
use std::{cell::RefCell, rc::Rc};
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let rc = if let Some(rc) = root { rc } else { return true };
    let r = rc.borrow();
    is_mirror(&r.left, &r.right)
}

pub fn is_mirror(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(l), Some(r)) => {
            let l = l.borrow();
            let r = r.borrow();
            l.val == r.val && is_mirror(&l.left, &r.right) && is_mirror(&l.right, &r.left)
        }
        _ => false,
    }
}
