use std::cmp::Ordering::{Equal, Greater, Less};

use data::leetcode::Tree;

pub fn search_bst(root: Tree, val: i32) -> Tree {
    let mut cur = root;
    loop {
        if let Some(next) = {
            let u = cur.as_ref()?;
            let r = u.borrow();
            match val.cmp(&r.val) {
                Less => Some(r.left.clone()),
                Equal => None,
                Greater => Some(r.right.clone()),
            }
        } {
            cur = next
        } else {
            return cur;
        }
    }
}
