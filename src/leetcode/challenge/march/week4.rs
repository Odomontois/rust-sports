use std::collections::{HashSet, VecDeque};
use std::iter::once;
use std::iter::repeat;
use std::vec;
use std::{collections::HashMap, usize};

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

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if heights.is_empty() || heights[0].is_empty() {
        return Vec::new();
    }
    let mut seen = vec![vec![0; heights[0].len()]; heights.len()];

    walk(&mut seen, 1, -1, -1, &heights);
    walk(&mut seen, 2, heights.len() as i32, heights[0].len() as i32, &&heights);
    seen.into_iter()
        .enumerate()
        .flat_map(|(i, sv)| {
            sv.into_iter()
                .enumerate()
                .filter_map(move |(j, s)| if s == 3 { Some(vec![i as i32, j as i32]) } else { None })
        })
        .collect()
}

fn near(i: i32, n: usize) -> impl Iterator<Item = i32> {
    once(i - 1)
        .chain(once(i + 1))
        .chain(once(i))
        .filter(move |&x| x >= 0 && x < n as i32)
}

fn neighbors(i: i32, j: i32, n: usize, m: usize) -> impl Iterator<Item = (i32, i32)> {
    near(i, n)
        .flat_map(move |i| near(j, m).map(move |j| (i, j)))
        .filter(move |&(x, y)| (x - i + y - j).abs() == 1)
}

fn walk(seen: &mut Vec<Vec<u8>>, bit: u8, vert: i32, hor: i32, heights: &Vec<Vec<i32>>) {
    let mut q = VecDeque::new();

    let n = heights.len();
    let m = heights[0].len();

    for i in 0..heights.len() {
        q.push_back((i as i32, hor, 0));
    }
    for i in 0..heights[0].len() {
        q.push_back((vert, i as i32, 0));
    }
    while let Some((i, j, h)) = q.pop_front() {
        for (x, y) in neighbors(i, j, n, m) {
            let h1 = heights[x as usize][y as usize];
            let s = &mut seen[x as usize][y as usize];
            if h1 >= h && (*s & bit == 0) {
                *s |= bit;
                q.push_back((x, y, h1));
            }
        }
    }
}

#[test]
fn test_walk() {
    fn check(map: &str, exp: &[[i32; 2]]) {
        let xs = map
            .split("\n")
            .map(str::trim)
            .map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect())
            .collect();
        assert_eq!(
            pacific_atlantic(xs).into_iter().collect::<HashSet<_>>(),
            exp.iter().map(|v| v.to_vec()).collect()
        )
    }

    check(
        "12235
             32344
             24531
             67145
             51124",
        &[[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]],
    )
}

fn char_count(s: &str) -> impl Iterator<Item = (usize, i8)> + '_ {
    let mut current = [0i8; 26];
    s.chars().map(|c| c as usize - 'a' as usize).map(move |x| {
        current[x] += 1;
        (x, current[x])
    })
}
pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    let mut total = [0i8; 26];
    for b in &b {
        for (ch, occ) in char_count(b) {
            total[ch] = total[ch].max(occ);
        }
    }
    let req = total.iter().filter(|&&x| x != 0).count();
    let universal = |a: &String| char_count(a).filter(|&(ch, occ)| occ == total[ch]).count() == req;
    a.into_iter().filter(universal).collect()
}

#[test]
fn check_kek() {
    fn tinfo<T>() {
        println!("{}: {}", std::any::type_name::<T>(), std::mem::size_of::<T>(),)
    }
    tinfo::<[i8; 26]>();
    tinfo::<Option<()>>();
    tinfo::<()>();
}

pub fn count_substrings(s: String) -> i32 {
    let b = s.into_bytes();
    (0..2 * b.len() + 1)
        .map(|i| {
            let (s, e) = (i / 2, (i + 1) / 2);
            (0..)
                .take_while(|&k| k <= s && e + k < b.len() && b[s - k] == b[e + k])
                .count() as i32
        })
        .sum()
}

pub fn original_digits(s: String) -> String {
    let mut chars = [0; 26];
    let mut digs = [0; 10];
    let idx = |c: char| c as usize - 'a' as usize;
    s.chars().for_each(|c| chars[idx(c)] += 1);
    let mut calc = |d: usize, word: &str, c: char| {
        let k = chars[idx(c)];
        digs[d] = k;
        word.chars().for_each(|c| chars[idx(c)] -= k)
    };
    calc(0, "zero", 'z');
    calc(2, "two", 'w');
    calc(6, "six", 'x');
    calc(8, "eight", 'g');
    calc(4, "four", 'u');
    calc(1, "one", 'o');
    calc(7, "seven", 's');
    calc(3, "three", 'h');
    calc(5, "five", 'v');
    calc(9, "nine", 'i');
    (0..=9)
        .flat_map(|d| repeat(('0' as u8 + d as u8) as char).take(digs[d]))
        .collect()
}

pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
    envelopes.sort_by_key(|e| (e[0], -e[1]));
    let mut seq = vec![];
    for env in envelopes {
        let i = seq.binary_search(&env[1]).unwrap_or_else(|e| e);
        if i == seq.len() {
            seq.push(env[1])
        }
        seq[i] = env[1];
    }
    seq.len() as i32
}

#[test]
fn envs_test() {
    fn check(xs: &[[i32; 2]], exp: i32) {
        assert_eq!(max_envelopes(xs.into_iter().map(|v| v.to_vec()).collect()), exp)
    }
    check(
        &[
            [2, 1],
            [4, 1],
            [6, 2],
            [8, 3],
            [10, 5],
            [12, 8],
            [14, 13],
            [16, 21],
            [18, 34],
            [20, 55],
        ],
        9,
    );
    check(&[[5, 4], [6, 4], [6, 7], [2, 3]], 3);
    check(&[[1, 1], [1, 1], [1, 1]], 1);
    check(&[[46, 89], [50, 53], [52, 68], [72, 45], [77, 81]], 3);
}
