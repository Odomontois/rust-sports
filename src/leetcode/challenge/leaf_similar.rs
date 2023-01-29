use data::leetcode::TreeNode;

struct Solution;

use std::{cell::RefCell, iter::from_fn, mem::swap, rc::Rc};

type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn leaf_similar(root1: Tree, root2: Tree) -> bool {
        leafs(root1).eq(leafs(root2))
    }
}

mod boxes {
    use std::iter::once;

    use super::Tree;

    fn leafs<'a>(root: Tree) -> impl Iterator<Item = i32> + 'a {
        root.into_iter().flat_map::<Box<dyn Iterator<Item = i32>>, _>(|rc| {
            let r = rc.borrow();
            if r.left.is_none() && r.right.is_none() {
                Box::new(once(r.val))
            } else {
                Box::new(leafs(r.left.clone()).chain(leafs(r.right.clone())))
            }
        })
    }
}

fn as_leaf(node: &TreeNode) -> Option<i32> {
    (node.left.is_none() && node.right.is_none()).then(|| node.val)
}

fn rotate(root: &mut Tree) -> Option<i32> {
    let mut right_node: Rc<RefCell<TreeNode>>;
    let node = root.as_ref()?;

    while node.borrow().left.is_none() {
        let leaf = as_leaf(&*node.borrow());
        if let Some(v) = leaf {
            *root = None;
            return Some(v);
        }
        right_node = node.borrow_mut().right.take()?;
        swap(&mut *node.borrow_mut(), &mut *right_node.borrow_mut())
    }

    let mut node = node.borrow_mut();

    while let Some(cr) = node.left.take() {
        let mut child = cr.borrow_mut();
        let leaf = as_leaf(&*child);
        if let Some(v) = leaf {
            let right = node.right.take();
            drop(node);
            *root = right;
            return Some(v);
        }
        let right = node.right.take();
        let mid = child.right.take();

        let link = right.is_some() || mid.is_some();

        let left = child.left.take();
        node.left = left;
        child.left = mid;
        child.right = right;

        swap(&mut child.val, &mut node.val);

        drop(child);
        if link {
            node.right = Some(cr);
        }
    }
    None
}

fn leafs(mut root: Tree) -> impl Iterator<Item = i32> {
    from_fn(move || rotate(&mut root))
}

#[cfg(test)]
use data::leetcode::IntoTree;

#[cfg(test)]
use itertools::Itertools;

#[test]
fn example1() {
    let t = (3, (5, 6, (2, 7, 4)), (1, 9, 8)).tree();
    assert_eq!(vec![6, 7, 4, 9, 8], leafs(t).collect_vec())
}

#[test]
fn example2() {
    let t = (3, (5, 6, 7), (1, 4, (2, 9, 8))).tree();
    assert_eq!(vec![6, 7, 4, 9, 8], leafs(t).collect_vec())
}

#[test]
fn example3() {
    let t = (1, 2, 3).tree();
    assert_eq!(vec![2, 3], leafs(t).collect_vec())
}

#[test]
fn example4() {
    let t = (1, 2, None).tree();
    assert_eq!(vec![2], leafs(t).collect_vec())
}

#[test]
fn example5() {
    let t = (1, (2, 3, None), None).tree();
    assert_eq!(vec![3], leafs(t).collect_vec())
}
