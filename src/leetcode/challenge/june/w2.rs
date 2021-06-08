use std::collections::HashMap;

use crate::leetcode::data::{tree, Tree};

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Tree {
    let n = inorder.len();
    Build::new(preorder, inorder).calc(0, 0, n)
}

struct Build {
    preorder: Vec<i32>,
    inorder: Vec<i32>,
    index: HashMap<i32, usize>,
}

impl Build {
    fn new(preorder: Vec<i32>, inorder: Vec<i32>) -> Self {
        Self {
            preorder,
            index: inorder.iter().enumerate().map(|(i, &x)| (x, i)).collect(),
            inorder,
        }
    }

    fn calc(&self, pix: usize, iix: usize, eix: usize) -> Tree {
        if iix == eix {
            return None;
        }
        let root = self.preorder[pix];
        let rix = self.index[&root];
        tree(
            root,
            self.calc(pix + 1, iix, rix),
            self.calc(pix + 1 + rix - iix, rix + 1, eix),
        )
    }
}
