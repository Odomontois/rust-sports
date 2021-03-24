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

pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
    let mut singles = vec![0u64; 101];
    let mut doubles = vec![0u64; 201];
    (arr.iter()
        .map(|&x| {
            let res = if x > target {
                return 0;
            } else if target - x > 200 {
                0
            } else {
                doubles[(target - x) as usize]
            };
            for (y, &c) in singles.iter().enumerate() {
                doubles[x as usize + y] += c
            }
            singles[x as usize] += 1;
            res
        })
        .sum::<u64>()
        % 1_000_000_007) as i32
}

pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut bxs: Vec<_> = (0..b.len() as u32).collect();
    bxs.sort_by_key(|&i| b[i as usize]);
    let mut bxi = bxs.into_iter().map(|i| (i, b[i as usize])).peekable();
    a.sort();
    let mut arest = vec![];
    let mut af = vec![-1; b.len()];
    for x in a {
        if let Some((i, _)) = bxi.peek().copied().filter(|&(_, b)| b < x) {
            bxi.next();
            af[i as usize] = x;
        } else {
            arest.push(x)
        }
    }
    for ox in &mut af {
        if *ox < 0 {
            *ox = arest.pop().unwrap()
        }
    }
    af
}

#[test]
fn check() {
    assert_eq!(
        advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
        vec![2, 11, 7, 15]
    );
    assert_eq!(
        advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
        vec![24, 32, 8, 12]
    );
}
