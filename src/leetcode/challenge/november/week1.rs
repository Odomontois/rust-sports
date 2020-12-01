#[allow(dead_code)]
pub fn max_power(s: String) -> i32 {
    s.chars().scan((1i32, '\0'), |(count, prev), c| {
        if *prev == c { *count += 1 } else { *count = 1 }
        *prev = c;
        Some(*count)
    }).max().unwrap_or(0)
}

#[allow(dead_code)]
pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
    let mut counts = [0, 0];
    position.into_iter().for_each(|x| counts[(x % 2) as usize] += 1);
    counts[0].min(counts[1])
}


struct Search { nums: Vec<i32>, threshold: i32 }

impl Search {
    fn go(&self, from: i32, to: i32) -> i32 {
        if (to - from) <= 1 { to } else {
            let m = (from + to) / 2;
            let res: i32 = self.nums.iter().map(|&n| (n + m - 1) / m).sum();
            if res <= self.threshold { self.go(from, m) } else { self.go(m, to) }
        }
    }
}

#[allow(dead_code)]
pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    Search { nums, threshold }.go(0, 2000_000)
}


use std::ops::Add;
use crate::leetcode::data::{ListNode, List};

struct ListNum(List);

impl ListNum {
    pub fn new(list: List) -> ListNum { ListNum(Self::reverse(list)) }
    fn unpack(list: List) -> Option<(i32, List)> {
        let bx = list?;
        let ListNode { val, next } = *bx;
        Some((val, next))
    }
    fn reverse(mut list: List) -> List {
        let mut res = None;
        while let Some((val, next)) = Self::unpack(list) {
            list = next;
            res = Some(Box::new(ListNode { val, next: res }))
        }
        res
    }
    fn into_list(self) -> List { Self::reverse(self.0) }
}

impl Add for ListNum {
    type Output = ListNum;

    fn add(self, rhs: Self) -> Self::Output {
        let mut rem = 0;
        let ListNum(mut x) = self;
        let ListNum(mut y) = rhs;
        let mut res = None;
        while x.is_some() || y.is_some() || rem > 0 {
            let (xv, x1) = Self::unpack(x).unwrap_or((0, None));
            x = x1;
            let (yv, y1) = Self::unpack(y).unwrap_or((0, None));
            y = y1;
            let v = xv + yv + rem;
            rem = v / 10;
            res = Some(Box::new(ListNode {
                val: v % 10,
                next: res,
            }))
        }
        ListNum::new(res)
    }
}

#[allow(dead_code)]
pub fn add_two_numbers(l1: List, l2: List) -> List {
    (ListNum::new(l1) + ListNum::new(l2)).into_list()
}

#[inline]
fn prepend(val: i32, next: List) -> List {
    Some(Box::new(ListNode { val, next }))
}

#[inline]
fn insert(val: i32, list: List) -> List {
    if let Some(b) = list {
        if val <= b.val { prepend(val, Some(b)) } else {
            prepend(b.val, insert(val, b.next))
        }
    } else { prepend(val, None) }
}

#[allow(dead_code)]
pub fn insertion_sort_list(mut head: List) -> List {
    let mut result = None;
    while let Some(b) = head {
        result = insert(b.val, result);
        head = b.next
    }
    result
}

