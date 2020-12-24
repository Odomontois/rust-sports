use crate::leetcode::data::{Tree, List};

struct Solution();

pub fn is_balanced(root: Tree) -> bool { balanced_depth(root).is_some() }

fn balanced_depth(root: Tree) -> Option<usize> {
    let rc = if let Some(rc) = root { rc } else { return Some(0); };
    let b = rc.borrow();
    let l = balanced_depth(b.left.clone())?;
    let r = balanced_depth(b.right.clone())?;
    if l > r + 1 || r > l + 1 { None } else { Some(l.max(r) + 1) }
}

pub fn next_greater_element(n: i32) -> i32 {
    let s = format!("{}", n);
    let mut digs: Vec<_> = s.chars().rev().collect();
    let mut prev = '0';
    for (i, &c) in digs.iter().enumerate() {
        if c >= prev { prev = c; } else {
            let j = digs.iter().position(|&x| x > c).unwrap();
            digs.swap(i, j);
            // println!("{} {} {:?}", i, j, digs);
            (&mut digs[0..i]).reverse();
            return digs.into_iter().rev().collect::<String>().parse::<i32>().unwrap_or(-1);
        }
    }
    -1
}

#[test]
fn nge() {
    println!("{}", next_greater_element(12));
    println!("{}", next_greater_element(102));
    println!("{}", next_greater_element(201));
    println!("{}", next_greater_element(132000));
    println!("{}", next_greater_element(320001));
    println!("{}", next_greater_element(320010));
    println!("{}", next_greater_element(320100));
    println!("{}", next_greater_element(321000));
}

impl Solution {
    pub fn swap_pairs(head: List) -> List {
        let mut node = *head?;
        let mut next = if let Some(n) = node.next { *n } else {
            return Some(Box::new(node));
        };
        node.next = Self::swap_pairs(next.next);
        next.next = Some(Box::new(node));
        Some(Box::new(next))
    }
}
