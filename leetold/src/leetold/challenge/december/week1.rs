use crate::data::leetcode::{List, Tree};
use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;

struct RandomChoose<R: Rng> { items: Vec<i32>, rng: R }

type Solution = RandomChoose<ThreadRng>;

impl Solution {
    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(mut head: List) -> Self {
        let mut items = Vec::new();
        while let Some(h) = head {
            items.push(h.val);
            head = h.next;
        }
        RandomChoose { items, rng: thread_rng() }
    }
}

impl<R: Rng> RandomChoose<R> {
    /** Returns a random node's value. */
    fn get_random(&mut self) -> i32 {
        let i = self.rng.gen_range(0, self.items.len());
        self.items[i]
    }
}

use std::mem::swap;

// pub type Tree = Option<Rc<RefCell<TreeNode>>>;


pub fn increasing_bst(root: Tree) -> Tree {
    let mut cur = root.clone();
    while let Some(rc) = cur {
        let rm = &mut *rc.borrow_mut();
        while let Some(lc) = rm.left.clone() {
            let lm = &mut *lc.borrow_mut();
            // [a - x - b] - y - c
            swap(&mut lm.right, &mut lm.left);
            // [b - x - a] - y - c
            swap(&mut lm.val, &mut rm.val);
            // [b - y - a] - x - c
            swap(&mut lm.right, &mut rm.right);
            // [b - y - c] - x - a
            swap(&mut rm.left, &mut rm.right);
            // a - x - [b - y - c]
        }
        cur = rm.right.clone();
    }
    root
}

pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
    let mut p = true;
    let mut pp = false;
    flowerbed.push(0);

    for f in flowerbed {
        if f == 0 && p && pp {
            n -= 1;
            pp = false;
        } else { pp = p; }
        p = f == 0;
        if n == 0 { return true; }
    }
    false
}

#[test]
fn can_place_test() {
    // assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    // assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    // assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2), false);
    assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 0, 0, 1], 2), true);
}


pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let ns = n as usize;
    let mut v = vec![vec![0i32; ns]; ns];
    if ns % 2 == 1 { v[ns / 2][ns / 2] = n * n }
    for i in 0..ns / 2 {
        for k in i..(ns - i - 1) {
            for (q, &(a, b)) in (&[(i, k), (k, ns - i - 1), (ns - i - 1, ns - k - 1), (ns - k - 1, i)]).into_iter().enumerate() {
                v[a][b] = ((ns - i) * 4 * i + 1 + k - i + q * (ns - 2 * i - 1)) as i32;
            }
        }
    }
    v
}

#[test]
fn test_generate_matrix() {
    println!("{:?}", generate_matrix(0));
    println!("{:?}", generate_matrix(1));
    println!("{:?}", generate_matrix(2));
    println!("{:?}", generate_matrix(3));
    println!("{:?}", generate_matrix(4));
    println!("{:?}", generate_matrix(5));
}

#[derive(Eq, Ord, PartialOrd, PartialEq)]
enum TreeSymbol<A>{
    Begin, End, Val(A)
}