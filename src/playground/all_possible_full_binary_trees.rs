use std::{cell::RefCell, rc::Rc};

use data::leetcode::TreeNode;
pub type Tree = Option<Rc<RefCell<TreeNode>>>;

fn tree(l: &Tree, r: &Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: l.clone(),
        right: r.clone(),
    })))
}

pub fn all_possible_fbt(n: i32) -> Vec<Tree> {
    let n = n as usize;
    let mut results = vec![vec![]; n + 1];
    results[1].push(tree(&None, &None));
    for size in 2..=n {
        let mut v = vec![];
        for lsize in 1..size {
            for left in &results[lsize] {
                for right in &results[size - lsize - 1] {
                    v.push(tree(left, right));
                }
            }
        }
        results[size] = v;
    }
    results.pop().unwrap_or_default()
}
