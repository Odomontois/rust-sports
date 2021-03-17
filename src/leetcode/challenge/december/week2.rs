use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;

use StackItem::*;

use crate::leetcode::data::{Tree, TreeNode};

pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
    let mut dd = vec![0; 60];
    for t in time {
        dd[(t % 60) as usize] += 1
    }
    (dd[0] * (dd[0] - 1) / 2) + (dd[30] * (dd[30] - 1) / 2) + (1..30).map(|i| dd[i] * dd[60 - i]).sum::<i32>()
}

#[test]
fn test_num_pairs() {
    println!("{}", num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]));
}

struct BSTIterator {
    stack: Vec<StackItem>,
}

enum StackItem {
    Val(i32),
    Node(Rc<RefCell<TreeNode>>),
}

impl BSTIterator {
    fn new(root: Tree) -> Self {
        BSTIterator {
            stack: root.into_iter().map(Node).collect(),
        }
    }

    fn next(&mut self) -> i32 {
        while let Some(i) = self.stack.pop() {
            match i {
                Val(x) => return x,
                Node(nr) => {
                    let n = nr.borrow();
                    for r in n.right.clone() {
                        self.stack.push(Node(r))
                    }
                    self.stack.push(Val(n.val));
                    for l in n.left.clone() {
                        self.stack.push(Node(l))
                    }
                }
            }
        }
        -1000
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    arr.windows(2).fold(0, |d, w| {
        if d == -1 {
            -1
        } else if w[0] > w[1] && d > 0 {
            2
        } else if w[0] < w[1] && d < 2 {
            1
        } else {
            -1
        }
    }) == 2
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    let mut p = None;
    let mut pp = None;
    for j in 0..nums.len() {
        if Some(nums[j]) == p && p == pp {
            continue;
        }
        pp = p;
        p = Some(nums[j]);
        if i != j {
            nums[i] = nums[j]
        }
        i += 1;
    }
    nums.drain(i..);
    nums.len() as i32
}

#[test]
fn check() {
    let mut v = vec![1, 1, 1, 2, 2, 3];
    remove_duplicates(&mut v);
    println!("{:?}", v)
}

pub fn subtree_with_all_deepest(root: Tree) -> Tree {
    look_subtree(root).0
}

pub fn look_subtree(root: Tree) -> (Tree, usize) {
    let rroot = if let Some(r) = root.clone() {
        r
    } else {
        return (None, 0);
    };
    let tree = rroot.borrow();
    let (l, ls) = look_subtree(tree.left.clone());
    let (r, rs) = look_subtree(tree.right.clone());
    match ls.cmp(&rs) {
        Ordering::Less => (r, rs + 1),
        Ordering::Equal => (root, rs + 1),
        Ordering::Greater => (l, ls + 1),
    }
}

pub fn max_coins(nums: Vec<i32>) -> i32 {
    let mut calc = CoinCalc::new(&nums);
    let res = calc.calc(CoinKey {
        from: 0,
        to: nums.len() as u16,
        left: 1,
        right: 1,
    });
    // for i in calc.cache.iter() { println!("{:?}", &i) };
    res
}

#[test]
fn test_max_coins() {
    println!("{}", max_coins(vec![3, 1, 5, 8]));
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct CoinKey {
    from: u16,
    to: u16,
    left: u8,
    right: u8,
}

#[derive(Debug)]
struct CoinCalc {
    cache: HashMap<CoinKey, i32>,
    nums: Vec<u8>,
}

impl CoinCalc {
    fn new(nums: &Vec<i32>) -> Self {
        CoinCalc {
            cache: HashMap::new(),
            nums: nums.iter().map(|&x| x as u8).collect(),
        }
    }

    fn calc(&mut self, key: CoinKey) -> i32 {
        let CoinKey { from, to, left, right } = key;
        if from == to {
            return 0;
        } else if from + 1 == to {
            return self.nums[from as usize] as i32 * left as i32 * right as i32;
        }
        if let Some(&v) = self.cache.get(&key) {
            return v;
        }
        let res = (from..to)
            .map(|i| {
                let x = self.nums[i as usize];
                let lkey = CoinKey {
                    from,
                    left,
                    to: i,
                    right: x,
                };
                let rkey = CoinKey {
                    to,
                    right,
                    from: i + 1,
                    left: x,
                };
                let val = x as i32 * left as i32 * right as i32;
                self.calc(lkey) + self.calc(rkey) + val
            })
            .max()
            .unwrap_or(0);

        self.cache.insert(key, res);
        res
    }
}

pub fn partition(s: String) -> Vec<Vec<String>> {
    fn is_palindrome(s: &str) -> bool {
        s.chars().zip(s.chars().rev()).all(|(c1, c2)| c1 == c2)
    }
    let pals = (0..s.len())
        .map(|i| {
            (i + 1..=s.len())
                .filter(|&j| is_palindrome(&s[i..j]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut res = vec![];
    let mut q = vec![(vec![], 0)];
    while let Some((v, i)) = q.pop() {
        if i == s.len() {
            res.push(v);
            continue;
        }
        for &j in &pals[i] {
            let mut v1 = v.clone();
            v1.push(s[i..j].to_string());
            q.push((v1, j));
        }
    }
    res
}

#[test]
fn check_partition() {
    println!("{:#?}", partition("aab".to_string()));
}

