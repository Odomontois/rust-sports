use std::{cell::RefCell, collections::HashMap, rc::Rc};

use data::leetcode::TreeNode;

type Tree = Option<Rc<RefCell<TreeNode>>>;
pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    Build(inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect()).build(0, &postorder)
}

struct Build(HashMap<i32, usize>);

impl Build {
    fn build(&self, i: usize, postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() {
            return None;
        }
        let n = postorder.len();
        let val = postorder[n - 1];
        let index = self.0[&val] - i;
        let left = self.build(i, &postorder[..index]);
        let right = self.build(i + index + 1, &postorder[index..n - 1]);
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}
