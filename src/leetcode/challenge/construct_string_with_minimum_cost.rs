use std::{collections::HashMap, iter::from_fn};

pub fn minimum_cost<S: AsRef<str>>(target: impl AsRef<str>, words: impl AsRef<[S]>, costs: impl AsRef<[i32]>) -> i32 {
    let (target, words, costs) = (target.as_ref(), words.as_ref(), costs.as_ref());
    let mut trie = Node::default();
    for (word, &cost) in words.iter().zip(costs) {
        trie.insert(word.as_ref(), cost);
    }
    let mut costs = vec![None; target.len() + 1];
    costs[0] = Some(0);

    for i in 0..costs.len() {
        let Some(cur) = costs[i] else { continue };
        for (j, cost) in trie.walk(&target[i..]) {
            min_or(&mut costs[i + j], cost + cur);
        }
    }
    costs[target.len()].unwrap_or(-1)
}

fn min_or<A: Ord>(x: &mut Option<A>, y: A) {
    *x = Some(if let Some(v) = x.take() { v.min(y) } else { y })
}

#[derive(Debug, Clone, Default)]
struct Node {
    children: HashMap<u8, Node>,
    cost: Option<i32>,
}

impl Node {
    fn insert(&mut self, word: &str, cost: i32) {
        let mut cur = self;
        for c in word.bytes() {
            cur = cur.children.entry(c).or_insert_with(|| Node::default());
        }
        min_or(&mut cur.cost, cost);
    }

    fn walk<'a>(&'a self, prefix: &'a str) -> impl Iterator<Item = (usize, i32)> + 'a {
        let mut cur = self;
        let mut i = 0;
        let mut prefix = prefix.bytes();
        from_fn(move || {
            cur = cur.children.get(&prefix.next()?)?;
            i += 1;
            Some(cur.cost.map(|c| (i, c)))
        })
        .flatten()
    }
}

#[test]
fn ex1() {
    assert_eq!(
        7,
        minimum_cost("abcdef", ["abdef", "abc", "d", "def", "ef"], [100, 1, 1, 10, 5])
    )
}

#[test]
fn ex2() {
    assert_eq!(-1, minimum_cost("aaaa", ["z", "zz", "zzz"], [1, 10, 100]))
}

#[test]
fn wa1() {
    assert_eq!(1, minimum_cost("r", ["r", "r", "r", "r"], [1, 6, 3, 3]))
}
