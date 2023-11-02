use std::{cell::RefCell, rc::Rc};

type Tree = Option<Rc<RefCell<TreeNode>>>;
use data::leetcode::TreeNode;

pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    #[derive(Default)]
    struct Res {
        good: i32,
        total: i32,
        sum: i32,
    }
    fn iter(t: &Tree) -> Res {
        let node = if let Some(x) = t {
            x.borrow()
        } else {
            return Res::default();
        };
        let lres = iter(&node.left);
        let rres = iter(&node.right);
        let total = lres.total + rres.total + 1;
        let sum = lres.sum + rres.sum + node.val;
        let good = lres.good + rres.good + if sum / total == node.val { 1 } else { 0 };
        Res { good, total, sum }
    }
    iter(&root).good
}
