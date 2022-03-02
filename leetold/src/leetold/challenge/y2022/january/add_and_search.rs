use std::collections::HashMap;

// https://leetcode.com/problems/design-add-and-search-words-data-structure/
#[derive(Default)]
struct WordDictionary {
    end: bool,
    next: HashMap<char, Box<WordDictionary>>,
}

impl WordDictionary {
    fn new() -> Self {
        Self::default()
    }

    fn add_word(&mut self, word: impl AsRef<str>) {
        let mut cur = self;
        for c in word.as_ref().chars() {
            cur = &mut **cur.next.entry(c).or_insert_with(|| Box::new(Self::new()));
        }
        cur.end = true
    }

    fn search(&self, word: impl AsRef<str>) -> bool {
        let mut stack = vec![(self, word.as_ref().chars())];
        while let Some((dict, mut suffix)) = stack.pop() {
            match suffix.next() {
                None if dict.end => return true,
                None => continue,
                Some('.') => {
                    for (_, next) in &dict.next {
                        stack.push((next, suffix.clone()))
                    }
                }
                Some(c) => {
                    for next in dict.next.get(&c) {
                        stack.push((next, suffix.clone()))
                    }
                }
            }
        }

        false
    }
}

#[test]
fn test1() {
    let mut dict = WordDictionary::new();
    dict.add_word("bad");
    dict.add_word("dad");
    dict.add_word("mad");
    assert!(!dict.search("pad")); // return False
    assert!(dict.search("bad")); // return True
    assert!(dict.search(".ad")); // return True
    assert!(dict.search("b.."));
}
