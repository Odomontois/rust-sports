use std::usize;

use crate::leetcode::data::Tree;

struct Search<'a> {
    nums: &'a [i32],
    from: i32,
    until: i32,
    sum: i32,
}
impl<'a> Clone for Search<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Copy for Search<'a> {}

impl<'a> Search<'a> {
    fn make(nums: &'a [i32], from: i32, until: i32) -> Self {
        let sum = nums.iter().filter(|&x| (from..until).contains(x)).sum();
        Self { nums, from, until, sum }
    }

    fn expected(self) -> i32 {
        (self.until + self.from - 1) * (self.until - self.from) / 2
    }
    fn normal(self) -> bool {
        self.sum == self.expected()
    }
    fn diff(self) -> i32 {
        self.expected() - self.sum
    }
    fn missing(self) -> Option<i32> {
        Some(self.diff()).filter(|&x| x > 0)
    }
    fn duplicate(self) -> Option<i32> {
        Some(-self.diff()).filter(|&x| x > 0)
    }
    fn split(self) -> (Self, Self) {
        let m = (self.from + self.until) / 2;
        (
            Self::make(self.nums, self.from, m),
            Self::make(self.nums, m, self.until),
        )
    }
}

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut search = Search::make(&nums, 1, nums.len() as i32 + 1);
    loop {
        let (l, r) = search.split();
        if l.normal() {
            search = r
        } else if r.normal() {
            search = l
        } else {
            return vec![
                l.duplicate().or(r.duplicate()).unwrap(),
                l.missing().or(r.missing()).unwrap(),
            ];
        }
    }
}

#[test]
fn check_find_error_nums() {
    assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3])
}

pub fn average_of_levels(root: Tree) -> Vec<f64> {
    let mut v = vec![];
    fill_average(&mut v, 0, root);
    v.into_iter().map(|(s, c)| s as f64 / c as f64).collect()
}

fn fill_average(v: &mut Vec<(i64, usize)>, i: usize, root: Tree) -> Option<()> {
    let r = root?;
    let node = r.borrow();
    if i >= v.len() {
        v.push((0, 0))
    }
    let k = &mut v[i];
    k.0 += node.val as i64;
    k.1 += 1;
    fill_average(v, i + 1, node.left.clone());
    fill_average(v, i + 1, node.right.clone());
    Some(())
}
