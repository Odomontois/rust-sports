use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use crate::data::leetcode::Tree;

pub fn can_merge(trees: Vec<Tree>) -> Tree {
    Merger::default().go(&trees)
}

#[derive(Default, Clone)]
struct Merger {
    roots: HashMap<i32, Tree>,
    seen: HashSet<i32>,
    count: usize,
}

impl Merger {
    fn check_bst(tree: Tree, rng: Range<i32>) -> Option<usize> {
        let t = if let Some(t) = tree { t } else { return Some(0) };
        let t = t.borrow();
        if !rng.contains(&t.val) {
            return None;
        }
        let l = Self::check_bst(t.left.clone(), rng.start..t.val)?;
        let r = Self::check_bst(t.right.clone(), t.val + 1..rng.end)?;
        Some(l + r + 1)
    }

    fn add_leaf(&mut self, leaf: &mut Tree) -> Option<()> {
        let n = if let Some(l) = leaf {
            l.borrow().val
        } else {
            return Some(());
        };
        if !self.seen.insert(n) {
            return None;
        }
        if let Some(root) = self.roots.remove(&n) {
            *leaf = root;
        }
        self.count += 1;
        Some(())
    }

    fn go(&mut self, trees: &[Tree]) -> Tree {
        for tree in trees {
            let n = tree.as_deref()?.borrow().val;
            self.roots.insert(n, tree.clone()).ok_or(()).err()?;
        }
        for tree in trees {
            let node = &mut *tree.as_ref()?.borrow_mut();
            self.add_leaf(&mut node.left)?;
            self.add_leaf(&mut node.right)?;
        }
        if self.roots.len() != 1 {
            return None;
        }
        let root = self.roots.values().cloned().next()?;
        root.clone()
            .filter(|_| Self::check_bst(root.clone(), i32::MIN..i32::MAX) == Some(self.count + 1))
    }
}

#[cfg(test)]
mod test {

    use crate::data::leetcode::{leaf, tree};

    use super::can_merge;

    #[test]
    fn test_can_merge() {
        assert_ne!(
            None,
            can_merge(vec![
                tree(2, leaf(1), None),
                tree(3, leaf(2), leaf(5)),
                tree(5, leaf(4), None),
            ])
        )
    }
}
