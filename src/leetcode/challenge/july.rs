use std::{cell::RefCell, rc::Rc};
pub type Tree = Option<Rc<RefCell<TreeNode>>>;

use data::leetcode::TreeNode;

pub fn lowest_common_ancestor(root: Tree, p: Tree, q: Tree) -> Tree {
    let x = p?.borrow().val;
    let y = q?.borrow().val;
    search(&root, x, y).ok()
}

type Done = Result<Rc<RefCell<TreeNode>>, bool>;
struct Comb<'a>(Done, &'a Rc<RefCell<TreeNode>>);
impl Comb<'_> {
    fn next<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Done,
    {
        let Comb(res, t) = self;
        let check = |e1, e2| if e1 && e2 { Ok(t.clone()) } else { Err(e1 || e2) };
        Comb(res.or_else(|e1| f().or_else(|e2| check(e1, e2))), self.1)
    }
}

fn search(root: &Tree, x: i32, y: i32) -> Done {
    let rt = root.as_ref().ok_or(false)?;
    let node = rt.borrow();
    Comb(search(&node.left, x, y), rt)
        .next(|| Err(node.val == x))
        .next(|| Err(node.val == y))
        .next(|| search(&node.right, x, y))
        .0
}
