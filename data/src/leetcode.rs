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

pub fn list_iter_mut(lst: &mut List) -> impl Iterator<Item = &mut i32> {
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
use std::fmt::Display;
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
        TreeNode { val, left: None, right: None }
    }
}

pub trait IntoTree {
    fn tree(self) -> Tree;
}

impl IntoTree for Tree {
    fn tree(self) -> Tree {
        self
    }
}

impl IntoTree for i32 {
    fn tree(self) -> Tree {
        (self, None, None).tree()
    }
}

impl<A: IntoTree, B: IntoTree> IntoTree for (i32, A, B) {
    fn tree(self) -> Tree {
        let (val, left, right) = self;
        let (left, right) = (left.tree(), right.tree());
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

pub type Tree = Option<Rc<RefCell<TreeNode>>>;

pub fn tree(val: i32, left: impl IntoTree, right: impl IntoTree) -> Tree {
    let (left, right) = (left.tree(), right.tree());
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

pub fn leaf(val: i32) -> Tree {
    tree(val, None, None)
}

#[test]
fn tree_eq() {
    assert_eq!((1, 2, 3).tree(), (1, 2, 3).tree())
}

pub struct Displayed<'a>(pub &'a Tree);

impl Display for Displayed<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Displayed(Some(x)) = self {
            write!(f, "{}", x.borrow())
        } else {
            write!(f, "*")
        }
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let TreeNode { val, left, right } = self;
        write!(f, "({val}, {}, {},)", Displayed(left), Displayed(right))
    }
}
