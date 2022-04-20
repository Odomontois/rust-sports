use std::{
    cell::RefCell,
    iter::{empty, from_fn, once, Peekable},
    rc::Rc,
};

use data::leetcode::{Tree, TreeNode};

struct BSTIterator(Peekable<Box<dyn Iterator<Item = i32>>>);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Tree) -> Self {
        Self(tree_it_box(root).unwrap_or(Box::new(empty())).peekable())
    }

    fn next(&mut self) -> i32 {
        self.0.next().unwrap_or(0)
    }

    fn has_next(&mut self) -> bool {
        self.0.peek().is_some()
    }
}

fn tree_it_box(root: Tree) -> Option<Box<dyn Iterator<Item = i32>>> {
    let rc = root.as_ref()?.borrow();
    let tree_it = |it: &Tree| tree_it_box(it.clone()).into_iter().flatten();
    let it = tree_it(&rc.left).chain(once(rc.val)).chain(tree_it(&rc.right));
    Some(Box::new(it))
}

fn tree_it2(root: Tree) -> impl Iterator<Item = i32> {
    let mut stack = vec![];
    let mut next = root;
    from_fn(move || {
        while let Some(rc) = next.take() {
            next = rc.borrow().left.clone();
            stack.push(rc);
        }
        let mid = stack.pop()?;
        let b = mid.borrow();
        next = b.right.clone();
        Some(b.val)
    })
}
