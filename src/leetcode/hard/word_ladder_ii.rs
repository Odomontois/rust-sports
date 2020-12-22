use std::collections::{HashMap, HashSet};

pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    fn keys(w: &String) -> impl Iterator<Item=(&str, &str)> {
        (0..w.len()).map(move |i| (&w[..i], &w[i + 1..]))
    }
    let mut words = HashMap::<(&str, &str), Vec<&String>>::new();
    for word in &word_list {
        for k in keys(word) {
            if let Some(v) = words.get_mut(&k) {
                v.push(word)
            } else {
                words.insert(k, vec![word]);
            }
        }
    }
    let mut seen = HashSet::<&String>::new();
    seen.insert(&begin_word);
    let mut q = vec![vec![&begin_word]];
    let mut res = vec![];
    let mut done = false;
    while !q.is_empty() && !done {
        let mut newq = vec![];
        for chain in q {
            let prev = chain.last().unwrap();
            for k in keys(prev) {
                for &word in words.get(&k).unwrap_or(&vec![]) {
                    if !seen.contains(&word) {
                        let mut new_chain = chain.clone();
                        new_chain.push(word);
                        if word == &end_word {
                            res.push(new_chain.into_iter().cloned().collect());
                            done = true;
                        } else {
                            newq.push(new_chain);
                        }
                    }
                }
            }
        }
        for chain in &newq {
            seen.insert(*chain.last().unwrap());
        }

        q = newq;
    }
    res
}

#[test]
fn test_word_ladder() {
    fn check(ws: &[&str], begin: &str, end: &str) {
        println!("{:?} {} {}", ws, begin, end);
        for chain in find_ladders(begin.to_string(), end.to_string(),
                                  ws.into_iter().map(|w| w.to_string()).collect(), ) {
            println!("{:?}", chain)
        }
    }
    check(&["hot", "dot", "dog", "lot", "log", "cog"], "hit", "cog");
    check(&["hot", "dot", "dog", "lot", "log"], "hit", "cog");
    check(&["a", "b", "c"], "a", "c");
}



