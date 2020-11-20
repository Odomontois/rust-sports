use State::*;

#[derive(Debug)]
enum State {
    Start,
    Climb(usize),
    Decline(usize),
}

#[allow(dead_code)]
pub fn longest_mountain(a: Vec<i32>) -> i32 {
    a.into_iter()
        .scan((Start, None::<i32>), |(state, prev), i| {
            let inc = prev.iter().any(|p| *p < i);
            let dec = prev.iter().any(|p| *p > i);
            *prev = Some(i);
            *state = match *state {
                Start => {
                    if inc {
                        Climb(2)
                    } else {
                        Start
                    }
                }
                Climb(n) => {
                    if inc {
                        Climb(n + 1)
                    } else if dec {
                        Decline(n + 1)
                    } else {
                        Start
                    }
                }
                Decline(n) => {
                    if dec {
                        Decline(n + 1)
                    } else if inc {
                        Climb(2)
                    } else {
                        Start
                    }
                }
            };
            Some(if let Decline(n) = *state { n } else { 0 })
        })
        .max()
        .unwrap_or(0) as i32
}

#[test]
fn mountain() {
    println!("{:?}", longest_mountain(vec![2, 2, 2]))
}

#[allow(dead_code)]
pub fn mirror_reflection(p: i32, q: i32) -> i32 {
    let g = gcd(p, q);
    if p / g % 2 == 0 {
        2
    } else if q / g % 2 == 0 {
        0
    } else {
        1
    }
}

use std::ops::Rem;

fn gcd<A: Rem<Output=A> + From<u8> + Ord + Clone>(a: A, b: A) -> A {
    if b > a {
        return gcd(b, a);
    };
    let r = a % b.clone();
    if r == 0.into() {
        b
    } else {
        gcd(b, r)
    }
}

use core::convert::TryFrom;
use std::iter::{once, repeat};

#[allow(dead_code)]
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let it = intervals
        .clone()
        .into_iter()
        .flat_map(|v| once((v[0] as u16, -1)).chain(once((v[1] as u16, 1))));
    dbg!(it.size_hint());

    let mut is: Vec<_> = intervals
        .into_iter()
        .flat_map(|v| <[i32; 2]>::try_from(v.as_slice()))
        .flat_map(|[x, y]| once((x as u16, -1)).chain(once((y as u16, 1))))
        .collect();
    is.sort();
    is.into_iter()
        .scan((0, None::<i32>), |(count, left), (x, op)| {
            if *count == 0 {
                *left = Some(x as i32)
            }
            *count -= op;
            Some(if let Some(l) = left.filter(|_| *count == 0) {
                Some(vec![l, x as i32])
            } else {
                None
            })
        })
        .flatten()
        .collect()
}

#[test]
fn merge_text() {
    fn to_vec2<V: IntoIterator<Item=[A; 2]>, A>(xs: V) -> Vec<Vec<A>> {
        xs.into_iter().map(|[x, y]| vec![x, y]).collect()
    }
    assert_eq!(
        merge(to_vec2(vec![[1, 3], [2, 6], [8, 10], [15, 18]])),
        to_vec2(vec![[1, 6], [8, 10], [15, 18]])
    )
}

#[allow(dead_code)]
pub fn decode_string(s: String) -> String {
    Expr::all_chars(parse_string(&mut s.chars())).collect()
}

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq, Hash)]
enum Expr {
    Single(char),
    Repeat { count: usize, exprs: Vec<Expr> },
}

impl Expr {
    fn all_chars(xs: Vec<Expr>) -> impl Iterator<Item=char> {
        xs.into_iter().flat_map(|expr| expr.into_chars())
    }

    fn into_chars(self) -> Box<dyn Iterator<Item=char>> {
        match self {
            Expr::Single(c) => Box::new(once(c)),
            Expr::Repeat { count, exprs } => Box::new(
                repeat(())
                    .take(count)
                    .flat_map(move |()| Self::all_chars(exprs.clone())),
            ),
        }
    }
}

fn parse_string(it: &mut impl Iterator<Item=char>) -> Vec<Expr> {
    let mut res = Vec::new();
    let mut num_state = None;
    while let Some(c) = it.next().filter(|c| *c != ']') {
        if let Some(d) = c.to_digit(10) {
            num_state = Some(num_state.unwrap_or(0) * 10 + d as usize)
        } else if c == '[' {
            let count = num_state.unwrap_or(1);
            num_state = None;
            let exprs = parse_string(it);
            res.push(Expr::Repeat { count, exprs })
        } else {
            res.push(Expr::Single(c))
        }
    }
    res
}

#[test]
fn parse_string_test() {
    println!("{:?}", parse_string(&mut "3[a]2[bc]".chars()));
    println!("{:?}", decode_string("3[a]2[bc]".to_string()));
    println!("{:?}", parse_string(&mut "3[a2[c]]".chars()));
    println!("{:?}", parse_string(&mut "2[abc]3[cd]ef".chars()));
}


#[allow(dead_code)]
pub fn search<A: Ord>(nums: Vec<A>, target: A) -> bool {
    if nums.is_empty() { return false; }
    match dbg!(search_pivot(&nums)) {
        None => target == nums[0],
        Some(None) => search_iter(&nums, &target, 0, nums.len()).is_some(),
        Some(Some(p)) => (match target.cmp(&nums[0]) {
            Less => search_iter(&nums, &target, p, nums.len()),
            Greater => search_iter(&nums, &target, 0, p),
            Equal => Some(0)
        }).is_some()
    }
}

#[test]
fn test_search() {
    // assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    // assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    // assert_eq!(search(vec![], 3), false);
    assert_eq!(search(vec![3, 1], 3), true);
}

use std::cmp::Ordering::*;

fn search_iter<A: Ord>(nums: &Vec<A>, target: &A, from: usize, to: usize) -> Option<usize> {
    if to - from == 1 { return Some(from).filter(|&i| &nums[i] == target); }
    let mid = (from + to) / 2;
    match nums[mid].cmp(target) {
        Less => search_iter(nums, target, mid, to),
        Greater => search_iter(nums, target, from, mid),
        Equal => Some(mid)
    }
}

fn search_pivot<A: Ord>(nums: &Vec<A>) -> Option<Option<usize>> {
    if nums.len() <= 1 { return Some(None); }
    let last = nums.len() - 1;
    match nums[0].cmp(&nums[last]) {
        Less => Some(None),
        Greater => Some(Some(search_pivot_strict(nums, 0, last))),
        Equal => search_pivot_nonstrict(nums, 0, last).map(|x| Some(x))
    }
}

// nums[from] == nums[to]
fn search_pivot_nonstrict<A: Ord>(nums: &Vec<A>, from: usize, to: usize) -> Option<usize> {
    dbg!(from..to);
    if to - from == 1 { return None; }
    let mid = (to + from) / 2;
    match nums[mid].cmp(&nums[from]) {
        Less => Some(search_pivot_strict(nums, from, mid)),
        Greater => Some(search_pivot_strict(nums, mid, to)),
        Equal => search_pivot_nonstrict(nums, from, mid).or(search_pivot_nonstrict(nums, mid, to)),
    }
}

// nums[from] > nums[to]
fn search_pivot_strict<A: Ord>(nums: &Vec<A>, from: usize, to: usize) -> usize {
    dbg!(from..to);
    if to - from == 1 { return to; }
    let mid = (to + from) / 2;
    if nums[mid] < nums[from] { search_pivot_strict(nums, from, mid) } else { search_pivot_strict(nums, mid, to) }
}
