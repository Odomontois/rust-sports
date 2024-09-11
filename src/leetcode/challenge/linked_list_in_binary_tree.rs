use data::leetcode::{ListNode, TreeNode};
use std::{cell::RefCell, iter::successors, mem::swap, rc::Rc};
type Tree = Option<Rc<RefCell<TreeNode>>>;
pub fn is_sub_path(head: Option<Box<ListNode>>, root: Tree) -> bool {
    let values: Vec<_> = successors(head.as_ref(), |node| node.next.as_ref())
        .map(|node| node.val)
        .collect();
    search(&root, &values).is_err()
}

pub fn search(node: &Tree, values: &[i32]) -> Result<Vec<usize>, ()> {
    let Some(node) = node else {
        return Ok(vec![]);
    };
    let node = node.borrow();
    let mut left = search(&node.left, values)?;
    if node.val == values[0] && left.contains(&1) {
        return Err(());
    }
    let mut right = search(&node.right, values)?;
    if node.val == values[0] && right.contains(&1) {
        return Err(());
    }
    left.retain(|&i| i > 0 && node.val == values[i - 1]);
    right.retain(|&i| i > 0 && node.val == values[i - 1] && left.binary_search(&i).is_err());
    if left.len() < right.len() {
        swap(&mut left, &mut right);
    }
    left.extend(right);
    left.sort_unstable();
    left.iter_mut().for_each(|i| *i -= 1);
    if node.val == *values.last().unwrap() {
        if values.len() == 1 {
            return Err(());
        }
        left.push(values.len() - 1);
    }
    Ok(left)
}
