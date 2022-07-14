use std::{
    collections::{BinaryHeap, VecDeque},
    iter::once,
};

pub fn maximum_minutes(mut grid: Vec<Vec<i32>>) -> i32 {
    let (n, m) = (grid.len(), grid[0].len());
    let mut q = VecDeque::new();
    for (i, line) in &mut grid.iter_mut().enumerate() {
        for (j, cell) in line.iter_mut().enumerate() {
            match *cell {
                2 => *cell = -1,
                1 => q.push_back((i, j, 1)),
                0 => *cell = 2000_000_000,
                _ => {}
            }
        }
    }
    while let Some((i, j, t)) = q.pop_front() {
        for (x, y) in neihbors(i, j, n, m) {
            if grid[x][y] == 2000_000_000 {
                grid[x][y] = t + 1;
                q.push_back((x, y, t + 1));
            }
        }
    }

    let mut bq: BinaryHeap<_> = once((grid[0][0] - 2, 3, 0, 0)).collect();
    grid[0][0] = -2;
    let home = (n - 1, m - 1);
    while let Some((s, k, i, j)) = bq.pop() {
        if (i, j) == home {
            return (1000_000_000).min(s);
        }
        for (x, y) in neihbors(i, j, n, m) {
            let cheat = if (x, y) == home { 1 } else { 0 };
            if grid[x][y] + cheat >= k {
                let t = (grid[x][y] - k + cheat).min(s);
                grid[x][y] = -2;
                bq.push((t, k + 1, x, y))
            }
        }
    }
    -1
}

const ADDS: [[usize; 2]; 4] = [[0, 1], [2, 1], [1, 0], [1, 2]];
fn neihbors(i: usize, j: usize, n: usize, m: usize) -> impl IntoIterator<Item = (usize, usize)> {
    let check = move |(x, y)| (x > 0 && y > 0 && x <= n && y <= m).then(|| (x - 1, y - 1));
    ADDS.iter().map(move |&[dx, dy]| (i + dx, j + dy)).filter_map(check)
}
