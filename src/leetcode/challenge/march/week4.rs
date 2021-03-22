use std::collections::{HashMap, HashSet};

pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    let exact: HashSet<_> = wordlist.iter().cloned().collect();
    let lb: HashMap<_, _> = wordlist.iter().rev().map(|s| (s.to_lowercase(), s.clone())).collect();
    let vow: HashMap<_, _> = wordlist.into_iter().rev().map(|s| (vowel_pat(&s), s)).collect();
    queries
        .into_iter()
        .map(|w| {
            if exact.contains(&w) {
                w
            } else {
                lb.get(&w.to_lowercase())
                    .or_else(|| vow.get(&vowel_pat(&w)))
                    .cloned()
                    .unwrap_or("".to_string())
            }
        })
        .collect()
}

fn vowel_pat(s: &str) -> String {
    s.chars()
        .map(|c| c.to_ascii_lowercase())
        .map(|c| if "aeiou".contains(c) { '_' } else { c })
        .collect()
}
