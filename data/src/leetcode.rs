#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    pub fn single(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_slice(xs: &[i32]) -> List {
        let mut cur = None;
        for &x in xs.iter().rev() {
            cur = cons(x, cur);
        }
        cur
    }
}

pub fn list(xs: &[i32]) -> List {
    ListNode::from_slice(xs)
}

pub type List = Option<Box<ListNode>>;

fn cons(val: i32, next: List) -> List {
    Some(Box::new(ListNode { val, next }))
}

pub fn list_iter_mut(lst: &mut List) -> impl Iterator<Item=&mut i32>{
    ListIterMut(Some(lst))
}

pub struct ListIterMut<'a>(pub Option<&'a mut List>);

impl<'a> Iterator for ListIterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        let taken = self.0.take()?;
        let list = taken.as_mut()?;
        self.0 = Some(&mut list.next);
        Some(&mut list.val)
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub type Tree = Option<Rc<RefCell<TreeNode>>>;

pub fn tree(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

pub fn leaf(val: i32) -> Tree{
    tree(val, None, None)
}
