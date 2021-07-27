use std::{cell::RefCell, rc::Rc};
pub type Tree = Option<Rc<RefCell<TreeNode>>>;

use data::leetcode::TreeNode;

pub fn lowest_common_ancestor(root: Tree, p: Tree, q: Tree) -> Tree {
    let x = p?.borrow().val;
    let y = q?.borrow().val;
    search(&root, x, y).ok()
}

type Done = Result<Rc<RefCell<TreeNode>>, bool>;
struct Comb<'a>(Done, &'a Rc<RefCell<TreeNode>>);
impl Comb<'_> {
    fn next<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Done,
    {
        let Comb(res, t) = self;
        let check = |e1, e2| if e1 && e2 { Ok(t.clone()) } else { Err(e1 || e2) };
        Comb(res.or_else(|e1| f().or_else(|e2| check(e1, e2))), self.1)
    }
}

fn search(root: &Tree, x: i32, y: i32) -> Done {
    let rt = root.as_ref().ok_or(false)?;
    let node = rt.borrow();
    Comb(search(&node.left, x, y), rt)
        .next(|| Err(node.val == x))
        .next(|| Err(node.val == y))
        .next(|| search(&node.right, x, y))
        .0
}

use rand::{
    prelude::{SliceRandom, ThreadRng},
    thread_rng,
};
struct Solution(Vec<i32>, ThreadRng);

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self(nums, thread_rng())
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.0.clone()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        let mut q = self.0.clone();
        q.shuffle(&mut self.1);
        q
    }
}

fn combine<S: Copy, I>(f: impl Fn(S, I) -> S) -> impl Fn(&mut S, I) -> Option<S> {
    move |s, i| {
        *s = f(*s, i);
        Some(*s)
    }
}

pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
    nums.iter()
        .copied()
        .rev()
        .skip(1)
        .chain(vec![0])
        .zip(nums.iter().copied().scan(-1, combine(i32::max)))
        .position(|(am, y)| y <= am)
        .map_or(0, |x| x as i32 + 1)
}

mod prune_tree;
mod three_sum_closest;
