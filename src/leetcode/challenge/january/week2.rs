use std::collections::{HashMap, HashSet, VecDeque};

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
    let mut q: VecDeque<_> = vec![(&begin_word, 0)].into_iter().collect();
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

    check("hit", "cog", &["hot","dot","dog","lot","log","cog"], 5);
}