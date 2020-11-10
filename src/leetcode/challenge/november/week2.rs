use std::rc::Rc;
use std::cell::RefCell;

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

type Tree = Option<Rc<RefCell<TreeNode>>>;

#[allow(dead_code)]
fn tilt_sum(root: Tree) -> (i32, i32) {
    if let Some(r) = root {
        let node = r.borrow();
        let (lsum, ltilt) = tilt_sum(node.left.clone());
        let (rsum, rtilt) = tilt_sum(node.right.clone());
        (node.val + lsum + rsum, (lsum - rsum).abs() + ltilt + rtilt)
    } else { (0, 0) }
}

#[allow(dead_code)]
pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    tilt_sum(root).1
}

#[derive(Copy, Clone)]
struct MaxDiff { max: i32, min: i32, diff: i32 }

impl MaxDiff {
    fn calc(root: Tree) -> Option<Self> {
        let r = root?;
        let node = r.borrow();
        let left = Self::calc(node.left.clone());
        let right = Self::calc(node.right.clone());
        let children: Vec<_> = left.into_iter().chain(right.into_iter()).collect();
        let min = children.iter().fold(node.val, |x, c| x.min(c.min));
        let max = children.iter().fold(node.val, |x, c| x.max(c.max));
        let my_diff = (node.val - min).max(max - node.val);
        let diff = children.iter().fold(my_diff, |x, c| x.max(c.diff));

        Some(MaxDiff { max, min, diff })
    }
}

#[allow(dead_code)]
pub fn max_ancestor_diff(root: Tree) -> i32 {
    MaxDiff::calc(root).map(|x| x.diff).unwrap_or(0)
}
#[allow(dead_code)]
pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    a.into_iter().map(|v| v.into_iter().rev().map(|i| 1 - i).collect()).collect()
}