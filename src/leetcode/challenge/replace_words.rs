use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::mem::{swap, take};

pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let mut trie = Trie { prefix: "", end: false, children: HashMap::new() };
    for word in &dictionary {
        trie.insert(word);
    }
    // let mut res = String::new();
    let mut res: String = sentence
        .split(' ')
        .flat_map(|word| {
            if let Some(u) = trie.find(word) {
                [&word[..u], " "]
            } else {
                [word, " "]
            }
        })
        .collect();
    res.pop();
    res
}

#[derive(Debug)]
pub struct Trie<'a> {
    prefix: &'a str,
    end: bool,
    children: HashMap<char, Trie<'a>>,
}

impl<'a> Trie<'a> {
    fn insert_child(&mut self, mut word: &'a str) -> Option<(&mut Self, &'a str)> {
        let Some(first) = word.chars().next() else {
            self.end = true;
            return None;
        };
        word = &word[1..];
        match self.children.entry(first) {
            Entry::Occupied(e) => return Some((e.into_mut(), word)),
            Entry::Vacant(v) => v.insert(Trie { prefix: word, end: true, children: HashMap::new() }),
        };
        None
    }
    fn find(&self, mut word: &str) -> Option<usize> {
        let mut node = self;
        let mut res = 0;
        loop {
            let common = node.common(word);
            res += common;
            if common < node.prefix.len() {
                return None;
            }
            if node.end {
                return Some(res);
            }
            let first = word[common..].chars().next()?;
            res += 1;
            node = node.children.get(&first)?;
            word = &word[common + 1..];
        }
    }
    fn common(&self, word: &str) -> usize {
        word.bytes()
            .zip(self.prefix.bytes())
            .take_while(|(x, y)| x == y)
            .count()
    }
    fn insert(&mut self, mut word: &'a str) {
        let mut node = self;
        loop {
            let common = node.common(word);
            word = &word[common..];
            let Some(first) = node.prefix[common..].chars().next() else {
                let Some(next) = node.insert_child(word) else {
                    return;
                };
                (node, word) = next;
                continue;
            };
            let mut children = HashMap::with_capacity(2);
            swap(&mut children, &mut node.children);
            let new = Trie { prefix: &node.prefix[common + 1..], end: node.end, children };
            node.prefix = &node.prefix[..common];
            node.children.insert(first, new);
            node.end = false;
            node.insert_child(word);
            return;
        }
    }
}
