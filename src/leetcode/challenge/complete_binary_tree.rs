use std::{cell::RefCell, rc::Rc};

use data::leetcode::TreeNode;
type Tree = Option<Rc<RefCell<TreeNode>>>;
pub fn is_complete_tree(root: Tree) -> bool {
    complete_level(&root).is_some()
}

fn complete_level(root: &Tree) -> Option<(bool, usize)> {
    let rc = if let Some(rc) = root {
        rc
    } else {
        return Some((true, 0));
    };
    let r = rc.borrow();
    let (lf, ll) = complete_level(&r.left)?;
    let (rf, rl) = complete_level(&r.right)?;
    if ll == rl && rf {
        Some((lf, ll + 1))
    } else if ll == rl + 1 && lf {
        Some((false, rl + 1))
    } else {
        None
    }
}
