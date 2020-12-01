use std::collections::VecDeque;
use crate::leetcode::data::{List, ListNode};

#[allow(dead_code)]
pub fn max_repeating(sequence: String, word: String) -> i32 {
    fn how_many(s: &str, w: &str) -> i32 {
        if s.starts_with(w) { 1 + how_many(&s[w.len()..], w) } else { 0 }
    }

    (0..sequence.len()).map(|i| how_many(&sequence[i..], word.as_str())).max().unwrap_or(0)
}


struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge_in_between(list1: List, a: i32, b: i32, list2: List) -> List {
        fn concat_rev(l1: List, l2: List) -> List {
            if let Some(lb) = l1 {
                let ListNode { val, next } = *lb;
                concat_rev(next, Some(Box::new(ListNode { val, next: l2 })))
            } else { l2 }
        }
        fn rev(l: List) -> List { concat_rev(l, None) }

        fn remove(l1: List, b: i32, l2: List) -> List {
            if b == -1 { return concat_rev(rev(l2), l1); }
            let ListNode { next, .. } = *(l1?);
            remove(next, b - 1, l2)
        }

        if a == 0 { return remove(list1, b, list2); }

        let ListNode { val, next } = *(list1?);
        Some(Box::new(ListNode { val, next: Self::merge_in_between(next, a - 1, b - 1, list2) }))
    }
}


#[derive(Debug, Clone)]
struct FrontMiddleBackQueue {
    left: VecDeque<i32>,
    right: VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    #[allow(dead_code)]
    fn new() -> Self { FrontMiddleBackQueue { left: VecDeque::new(), right: VecDeque::new() } }

    fn fix(&mut self) {
        if self.right.len() < self.left.len() + 1 {
            for x in self.left.pop_back() {
                self.right.push_front(x)
            }
        }
        if self.left.len() < self.right.len() {
            for x in self.right.pop_front() {
                self.left.push_back(x)
            }
        }
    }

    #[allow(dead_code)]
    fn push_front(&mut self, val: i32) {
        self.left.push_front(val);
        self.fix();
    }

    #[allow(dead_code)]
    fn push_middle(&mut self, val: i32) {
        if self.left.len() > self.right.len() {
            for x in self.left.pop_back() {
                self.right.push_front(x)
            }
        }
        self.left.push_back(val);
        self.fix();
    }

    #[allow(dead_code)]
    fn push_back(&mut self, val: i32) {
        self.right.push_back(val);
        self.fix();
    }

    #[allow(dead_code)]
    fn pop_front(&mut self) -> i32 {
        let res = self.left.pop_front().unwrap_or(-1);
        self.fix();
        res
    }

    #[allow(dead_code)]
    fn pop_middle(&mut self) -> i32 {
        let res = self.left.pop_back().unwrap_or(-1);
        self.fix();
        res
    }

    #[allow(dead_code)]
    fn pop_back(&mut self) -> i32 {
        let res = self.right.pop_back().or_else(|| self.left.pop_back()).unwrap_or(-1);
        self.fix();
        res
    }
}

#[test]
fn test_fbmq() {
    let mut q = FrontMiddleBackQueue::new();
    q.push_front(1);
    q.push_front(2);
    q.push_front(3);
    q.push_front(4);
    println!("{:?}", q);
    println!("{}", q.pop_back());
    println!("{:?}", q);
    println!("{}", q.pop_back());
    println!("{:?}", q);
    println!("{}", q.pop_back());
    println!("{:?}", q);
    println!("{}", q.pop_back());
    println!("{:?}", q);
}

#[allow(dead_code)]
pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let mut desc = vec![0; nums.len()];
    let mut asc = vec![0; nums.len()];
    for i in 1..nums.len() {
        asc[i] = (0..i).filter(|&j| nums[j] < nums[i]).map(|j| asc[j] + 1).max().unwrap_or(0);
        let k = nums.len() - i - 1;
        desc[k] = (k + 1..nums.len()).filter(|&j| nums[j] < nums[k]).map(|j| desc[j] + 1).max().unwrap_or(0)
    }

    // println!("{:?} {:?}", asc, desc);

    (0..nums.len())
        .filter(|&i| asc[i] > 0 && desc[i] > 0)
        .map(|i| nums.len() - asc[i] - desc[i] - 1)
        .min()
        .unwrap_or(0) as i32
}

#[test]
fn test_mmr() {
    println!("{}", minimum_mountain_removals(vec![1, 3, 1]));
    println!("{}", minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]));
}

