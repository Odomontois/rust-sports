use crate::leetcode::data::Tree;

struct Solution;

impl Solution {
    pub fn trim_bst(root: Tree, low: i32, high: i32) -> Tree {
        let r = root?;
        let mut rb = r.borrow_mut();
        if rb.val < low {
            Self::trim_bst(rb.right.clone(), low, high)
        } else if rb.val > high {
            Self::trim_bst(rb.left.clone(), low, high)
        } else {
            rb.left = Self::trim_bst(rb.left.clone(), low, high);
            rb.right = Self::trim_bst(rb.right.clone(), low, high);
            drop(rb);
            Some(r)
        }
    }
}