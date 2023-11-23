use std::collections::{HashMap, HashSet};
pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    let begin: &str = &begin_word;
    let end: &str = &&end_word;
    fn keys(w: &str) -> impl Iterator<Item = String> + '_{
        (0..w.len()).map(move |i| format!("{}_{}", &w[..i], &w[i + 1..]))
    }
    let mut words = HashMap::<String, Vec<&str>>::new();
    for word in &word_list {
        for k in keys(word) {
            if let Some(v) = words.get_mut(&k) {
                v.push(word)
            } else {
                words.insert(k, vec![word]);
            }
        }
    }
    let mut seen = HashSet::<&str>::new();
    seen.insert(begin);
    let mut q = vec![begin];
    let mut prev = HashMap::<&str, Vec<&str>>::new();
    while !q.is_empty() && !seen.contains(&end) {
        let mut cur = HashSet::<&str>::new();
        for w in q {
            for k in keys(w) {
                for &next in words.get(&k).unwrap_or(&vec![]) {
                    if !seen.contains(&next) {
                        prev.entry(next).or_insert(vec![]).push(w);
                        cur.insert(next);
                    }
                }
            }
        }
        seen.extend(cur.iter().cloned());

        q = cur.into_iter().collect();
    }
    let mut res = vec![];
    unwind(&end_word, &begin_word, &prev, &mut res, &mut vec![end.to_string()]);
    res
}

fn unwind(
    end: &str,
    start: &str,
    prev: &HashMap<&str, Vec<&str>>,
    acc: &mut Vec<Vec<String>>,
    chain: &mut Vec<String>,
) {
    for w in prev.get(end).iter().cloned().flatten().cloned() {
        chain.push(w.to_string());
        if w == start {
            let mut c = chain.clone();
            c.reverse();
            acc.push(c);
        } else {
            unwind(w, start, prev, acc, chain)
        }
        chain.pop();
    }
}

#[cfg(test)]
fn check<const N: usize>(begin: &str, end: &str, word_list: &[&str], exp: &[[&str; N]]) {
    let res = find_ladders(
        begin.to_string(),
        end.to_string(),
        word_list.iter().map(|s| s.to_string()).collect(),
    );
    let exp = exp
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let ss = |mut vv: Vec<Vec<String>>| {
        for v in &mut vv {
            v.sort();
        }
        vv.sort();
        vv
    };
    assert_eq!(ss(res), ss(exp))
}

#[test]
fn test1() {
    check(
        "hit",
        "cog",
        &["hot", "dot", "dog", "lot", "log", "cog"],
        &[["hit", "hot", "dot", "dog", "cog"], ["hit", "hot", "lot", "log", "cog"]],
    );
}
