use std::{collections::HashMap, usize};

pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    points
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v[0] == x {
                Some(((y - v[1]).abs(), i))
            } else if v[1] == y {
                Some(((x - v[0]).abs(), i))
            } else {
                None
            }
        })
        .min()
        .map(|(_, i)| i as i32)
        .unwrap_or(-1)
}

pub fn check_powers_of_three(mut n: i32) -> bool {
    while n > 0 {
        if n % 3 == 2 {
            return false;
        }
        n /= 3;
    }
    true
}

pub fn beauty_sum(s: String) -> i32 {
    let mut counts = vec![[0u16; 26]];
    for (i, c) in s.chars().enumerate() {
        let mut cnt = counts[i];
        cnt[c as usize - 'a' as usize] += 1;
        counts.push(cnt);
    }
    let counts: &[_] = &counts;
    (1..=counts.len())
        .flat_map(|j| (0..j).map(move |i| beauty(&counts[i], &counts[j])))
        .sum()
}

fn beauty(start: &[u16], end: &[u16]) -> i32 {
    let cnts = || (0..26).map(|i| end[i] - start[i]).filter(|&x| x > 0);
    (cnts().max().unwrap_or(0) - cnts().min().unwrap_or(0)) as i32
}

pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let n = n as usize;
    let mut ranks = vec![0; n];
    let mut edge_count = HashMap::new();
    let edges: Vec<_> = edges
        .into_iter()
        .map(|v| (v[0] as usize - 1, v[1] as usize - 1))
        .map(|(i, j)| if i < j { (i, j) } else { (j, i) })
        .collect();
    for &(x, y) in &edges {
        ranks[x] += 1;
        ranks[y] += 1;
        *edge_count.entry((x, y)).or_insert(0) += 1;
    }
    let mut rank_count = HashMap::new();
    for &r in &ranks {
        *rank_count.entry(r).or_insert(0) += 1;
    }
    let mut prank_count = HashMap::new();
    for (&r1, &c1) in &rank_count {
        *prank_count.entry(2 * r1).or_insert(0) += c1 * (c1 - 1) / 2;
        for (&r2, &c2) in &rank_count {
            if r1 >= r2 {
                continue;
            }
            *prank_count.entry(r1 + r2).or_insert(0) += c1 * c2;
        }
    }
    for (&(i, j), cnt) in &edge_count {
        let wrong = ranks[i] + ranks[j];
        let correct = wrong - cnt;
        *prank_count.entry(wrong).or_insert(0) -= 1;
        *prank_count.entry(correct).or_insert(0) += 1;
    }
    let mut prank: Vec<_> = prank_count.into_iter().collect();
    prank.sort();
    prank.reverse();
    let mut acc = 0;
    for (_, v) in &mut prank {
        acc += *v;
        *v = acc;
    }
    prank.reverse();

    queries
        .into_iter()
        .map(|q| {
            let i = prank
                .binary_search_by_key(&q, |(k, _)| *k)
                .map(|i| i + 1)
                .unwrap_or_else(|x| x);
            if i == prank.len() {
                0
            } else {
                prank[i].1
            }
        })
        .collect()
}

#[test]
fn check_pairs() {
    assert_eq!(
        count_pairs(
            4,
            vec![vec![1, 2], vec![2, 4], vec![1, 3], vec![2, 3], vec![2, 1]],
            vec![2, 3],
        ),
        vec![6, 5]
    );

    assert_eq!(
        count_pairs(
            5,
            vec![
                vec![1, 5],
                vec![1, 5],
                vec![3, 4],
                vec![2, 5],
                vec![1, 3],
                vec![5, 1],
                vec![2, 3],
                vec![2, 5]
            ],
            vec![1, 2, 3, 4, 5],
        ),
        vec![10, 10, 9, 8, 6]
    );
}
