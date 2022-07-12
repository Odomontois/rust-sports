use std::{cell::RefCell, mem::swap, rc::Rc};

use data::leetcode::{Tree, TreeNode};

pub fn recover_tree(root: &Tree) {
    if let Some((f, s)) = find_swap(tree_it(root), |x| x.borrow().val) {
        let mut fc = f.borrow_mut();
        let mut sc = s.borrow_mut();
        swap(&mut fc.val, &mut sc.val)
    }
}

fn tree_it_box(root: Tree) -> Option<Box<dyn Iterator<Item = Rc<RefCell<TreeNode>>>>> {
    let rc = root.as_ref()?.borrow();
    let it = tree_it(&rc.left).chain(root.clone()).chain(tree_it(&rc.right));
    Some(Box::new(it))
}

fn tree_it(x: &Tree) -> impl Iterator<Item = Rc<RefCell<TreeNode>>> {
    tree_it_box(x.clone()).into_iter().flatten()
}

fn find_swap<A, B: Ord>(it: impl IntoIterator<Item = A>, f: impl for<'a> Fn(&'a A) -> B) -> Option<(A, A)> {
    let mut first = None;
    let mut second = None;

    let mut it = it.into_iter().map(|a| (f(&a), a));

    let (mut pb, mut pa) = it.next()?;

    for (b, a) in it {
        if pb > b {
            if let Some(f) = first {
                return Some((f, a));
            }
            first = Some(pa);
        } else if first.is_some() && second.is_none() {
            second = Some(pa);
        }
        pa = a;
        pb = b;
    }

    Some((first?, second.unwrap_or(pa)))
}
