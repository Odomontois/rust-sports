use std::fmt::{Debug, Formatter};

pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
    min_deviation(nums.into_iter()).unwrap_or(0)
}

fn min_deviation(mut nums: impl Iterator<Item=i32>) -> Option<i32> {
    let first = nums.next()?;
    let init: Vec<_> = variants(first).into_iter().map(Interval::new).collect();
    // println!("init={:?}", &init);
    let result = nums.map(variants).fold(init, handle);
    result.iter().map(Interval::deviation).min()
}

fn variants(mut n: i32) -> Vec<i32> {
    if n % 2 == 1 { return vec![n, n * 2]; }
    let mut res = vec![n];
    while n % 2 == 0 {
        n /= 2;
        res.push(n);
    }
    res.reverse();
    res
}

#[derive(Copy, Clone)]
struct Interval { from: i32, to: i32 }

impl Interval {
    fn new(x: i32) -> Interval { Interval { from: x, to: x } }

    fn with(&self, p: i32) -> Interval {
        if self.from > p { Interval { from: p, to: self.to } } else if self.to < p { Self { to: p, from: self.from } } else { *self }
    }

    fn deviation(&self) -> i32 { self.to - self.from }

    fn contains(&self, p: i32) -> bool { self.to >= p && self.from <= p }
}

impl Debug for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (&[self.from, self.to]).fmt(f)
    }
}


fn handle(intervals: Vec<Interval>, ps: Vec<i32>) -> Vec<Interval> {
    let mut i = 0;
    let mut res = Vec::new();
    let n = intervals.len();
    for (j, &p) in ps.iter().enumerate() {
        while i + 1 < n && intervals[i + 1].to <= p { i += 1; }
        if j + 1 < ps.len() && intervals[i].to >= ps[j + 1] { continue; }
        res.push(intervals[i].with(p));
        while i + 1 < n && intervals[i].from < p && (j + 1 == ps.len() || intervals[i + 1].to < ps[j + 1]) {
            i += 1;
            res.push(intervals[i].with(p));
            if i == n { break; }
        }
        if intervals[i].from <= p {
            if i + 1 == n { break; } else { i += 1 }
        }
    }

    res
}

#[test]
fn test() {
    println!("{}", minimum_deviation(vec![1, 2, 3, 4]));
    println!("{}", minimum_deviation(vec![4, 1, 5, 20, 3]));
    println!("{}", minimum_deviation(vec![2, 10, 8]));
    println!("{}", minimum_deviation(vec![15, 30, 17, 34, 15, 30, 17, 34]));
    println!("{}", minimum_deviation(vec![9, 15]));
    println!("{}", minimum_deviation(vec![399, 908, 648, 357, 693, 502, 331, 649, 596, 698]));
    // println!("{}", minimum_deviation(vec![1000001, 2000002, 2000003, 2000004, 2000005, 2000006, 2000007]));
}