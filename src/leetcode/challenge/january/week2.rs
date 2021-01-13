use std::collections::{HashMap, HashSet, VecDeque, BTreeSet};
use crate::leetcode::data::{List, ListNode};

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


