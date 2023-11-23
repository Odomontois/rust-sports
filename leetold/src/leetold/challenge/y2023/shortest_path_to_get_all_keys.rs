use std::{collections::VecDeque, ops::Deref};

struct Item {
    row: usize,
    col: usize,
    keys: usize,
    steps: i32,
}

pub fn shortest_path_all_keys<S: Deref<Target = str>>(grid: Vec<S>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let chars = grid.iter().flat_map(|s| s.chars());
    let k = chars.filter(|&c| c.is_ascii_lowercase()).count();
    let mut path = vec![vec![vec![None::<i32>; 1 << k]; n]; m];
    let mut rows = grid.iter().enumerate();
    let start_opt = rows.find_map(|(i, v)| v.chars().enumerate().find_map(move |(j, c)| (c == '@').then(|| (i, j))));
    let (si, sj) = start_opt.unwrap_or((0, 0));

    let mut q: VecDeque<_> = Some(Item {
        row: si,
        col: sj,
        keys: 0,
        steps: 0,
    })
    .into_iter()
    .collect();
    let key = |i: usize, j: usize, keys: usize| -> Option<usize> {
        let c = grid[i].chars().nth(j)?;
        if c == '#' || c.is_ascii_uppercase() && keys & 1 << (c as u8 - b'A') == 0 {
            return None;
        }
        Some(if c.is_ascii_lowercase() {
            keys | (1 << (c as u8 - b'a'))
        } else {
            keys
        })
    };
    while let Some(Item { row, col, keys, steps }) = q.pop_front() {
        let (i, j) = (row + 1, col + 1);
        let xs = [(i, j + 1), (i, j - 1), (i + 1, j), (i - 1, j)];
        let neighbor_filter = |&(x, y)| (x <= m && y <= n && x > 0 && y > 0).then(|| (x - 1, y - 1));
        let neighbors = xs.iter().filter_map(neighbor_filter);
        for (i, j) in neighbors {
            let new_keys = if let Some(k) = key(i, j, keys) {
                k
            } else {
                continue;
            };
            if path[i][j][new_keys].is_some() {
                continue;
            }
            if new_keys == (1 << k) - 1 {
                return steps + 1;
            }
            path[i][j][new_keys] = Some(steps + 1);
            q.push_back(Item {
                row: i,
                col: j,
                keys: new_keys,
                steps: steps + 1,
            });
        }
    }
    return -1;
}

#[test]
fn example1() {
    assert_eq!(8, shortest_path_all_keys(vec!["@.a.#", "###.#", "b.A.B"]));
}

#[test]
fn example2() {
    assert_eq!(6, shortest_path_all_keys(vec!["@..aA", "..B#.", "....b"]));
}

#[test]
fn example3() {
    assert_eq!(-1, shortest_path_all_keys(vec!["@Aa"]));
}
