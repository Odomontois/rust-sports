use crate::leetcode::data::Tree;


pub fn is_balanced(root: Tree) -> bool { balanced_depth(root).is_some() }

fn balanced_depth(root: Tree) -> Option<usize> {
    let rc = if let Some(rc) = root { rc } else { return Some(0); };
    let b = rc.borrow();
    let l = balanced_depth(b.left.clone())?;
    let r = balanced_depth(b.right.clone())?;
    if l > r + 1 || r > l + 1 { None } else { Some(l.max(r) + 1) }
}