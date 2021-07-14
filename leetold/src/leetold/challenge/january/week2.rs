use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::iter::{once};

use crate::data::leetcode::{List, ListNode};

pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let mut edges = HashMap::<_, Vec<_>>::new();
    for word in &word_list {
        for i in 0..word.len() {
            let z = (&word[..i], &word[i + 1..]);
            if let Some(v) = edges.get_mut(&z) {
                v.push(word);
            } else {
                edges.insert(z, vec![word]);
            }
        }
    }
    let mut seen: HashSet<_> = vec![&begin_word].into_iter().collect();
    let mut q: VecDeque<_> = vec![(&begin_word, 1)].into_iter().collect();
    while let Some((word, k)) = q.pop_front() {
        if word == &end_word { return k; }
        for i in 0..word.len() {
            for &next in edges.get(&(&word[..i], &word[i + 1..])).into_iter().flatten() {
                if seen.insert(next) {
                    q.push_back((next, k + 1));
                }
            }
        }
    }
    0
}

#[test]
fn ladder_test() {
    fn check(begin: &str, end: &str, words: &[&str], exp: i32) {
        assert_eq!(ladder_length(begin.to_string(), end.to_string(),
                                 words.iter().map(|&w| w.to_string()).collect(),
        ), exp)
    }

    check("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"], 5);
    println!("{} {}", true as i32, false as i32)
}


fn list(val: i32, next: List) -> List { Some(Box::new(ListNode { val, next })) }

pub fn add_two_numbers(l1: List, l2: List) -> List {
    fn go(x: List, y: List, overflow: i32) -> List {
        match (x, y) {
            (Some(bx), Some(by)) => {
                let s = bx.val + by.val + overflow;
                list(s % 10, go(bx.next, by.next, s / 10))
            }
            (None, ly) if overflow == 0 => ly,
            (lx, None) if overflow == 0 => lx,
            (None, ly) => go(list(overflow, None), ly, 0),
            (lx, None) => go(lx, list(overflow, None), 0),
        }
    }
    go(l1, l2, 0)
}

pub fn num_rescue_boats1(people: Vec<i32>, limit: i32) -> i32 {
    let mut people: BTreeSet<_> = people.into_iter().enumerate().map(|(x, y)| (y, x)).collect();
    let mut count = 0;
    while let Some(&(p, i)) = people.iter().rev().next() {
        count += 1;
        people.remove(&(p, i));
        if let Some(&k) = people.range(..=(limit - p, std::usize::MAX)).next() {
            people.remove(&k);
        }
    }
    count
}

pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort();
    let mut it = people.into_iter().peekable();
    let mut count = 0;
    while let Some(p) = it.next_back() {
        count += 1;
        if it.peek().iter().any(|&&q| q + p <= limit) {
            it.next();
        }
    }
    count
}


pub fn min_operations1(nums: Vec<i32>, x: i32) -> i32 {
    fn sums<'a>(xs: impl Iterator<Item=&'a i32> + 'a) -> impl Iterator<Item=i32> + 'a {
        once(0).chain(xs.scan(0, |s, &x| {
            *s += x;
            Some(*s)
        }))
    }
    let forward = sums(nums.iter()).enumerate().map(|(i, s)| (s, i)).collect::<HashMap<_, _>>();
    let n = nums.len() as i32;

    sums(nums.iter().rev()).enumerate().filter_map(|(i, s)|
        forward.get(&(x - s)).map(|&j| (i + j) as i32).filter(|&k| k <= n)
    ).min().unwrap_or(-1)
}

pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let forward = std::iter::once(0).chain(nums.iter().scan(0, |s, &x| {
        *s += x;
        Some(*s)
    })).enumerate();
    let s: i32 = nums.iter().copied().sum();
    let mut back = forward.clone().map(|(i, x)| (nums.len() - i, s - x)).peekable();
    let mut forward = forward.peekable();
    std::iter::from_fn(|| {
        let &(i, a) = forward.peek()?;
        let &(j, b) = back.peek()?;
        if a + b < x { forward.next() } else { back.next() };
        Some(Some((i + j) as i32).filter(|_| a + b == x && i + j <= nums.len()))
    }).flatten().min().unwrap_or(-1)
}

#[test]
fn min_op_test() {
    assert_eq!(min_operations(vec![1, 1, 4, 2, 3], 5), 2);
    assert_eq!(min_operations(vec![5, 6, 7, 8, 9], 4), -1);
    assert_eq!(min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
}