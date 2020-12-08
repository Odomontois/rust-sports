use crate::leetcode::data::Tree;

mod week1;
mod week2;

#[allow(dead_code)]
pub fn max_depth(root: Tree) -> i32 {
    fn iter(nr: &Tree) -> i32 {
        if let Some(nr) = nr {
            let nb = nr.borrow();
            iter(&nb.left).max(iter(&nb.right)) + 1
        } else { 0 }
    }
    iter(&root)
}