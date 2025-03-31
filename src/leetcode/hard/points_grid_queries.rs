pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let n = grid.len();
    let m = grid[0].len();
    let mut next = BinaryHeap::from_iter([(Reverse(grid[0][0]), 0, 0)]);
    let mut q = vec![];
    let mut cost = vec![(0, 0)];
    let mut cur = 0;
    let mut seen = vec![false; m * n];
    seen[0] = true;
    while let Some((Reverse(c), sx, sy)) = next.pop() {
        q.push((sx, sy));
        while matches!(next.peek(), Some((Reverse(p), _, _)) if *p == c) {
            if let Some((_, x, y)) = next.pop() {
                q.push((x, y));
            }
        }
        while let Some((x, y)) = q.pop() {
            let b = grid[x][y];
            if b > c {
                next.push((Reverse(b), x, y));
                continue;
            }
            cur += 1;
            for (dx, dy) in [(0, 1), (2, 1), (1, 0), (1, 2)] {
                let (x, y) = (x + dx, y + dy);
                if x <= n && y <= m && x > 0 && y > 0 && !seen[(x - 1) * m + y - 1] {
                    q.push((x - 1, y - 1));
                    seen[(x - 1) * m + y - 1] = true;
                }
            }
        }
        cost.push((c, cur));
    }
    let query = |q: i32| cost[cost.binary_search_by_key(&(q - 1), |p| p.0).unwrap_or_else(|e| e - 1)].1;
    queries.into_iter().map(query).collect()
}

#[cfg(test)]
fn check<const Q: usize>(grid: impl IntoIterator<Item = impl AsRef<[i32]>>, queries: [i32; Q], expect: [i32; Q]) {
    let grid = grid.into_iter().map(|x| x.as_ref().to_vec()).collect();
    assert_eq!(max_points(grid, queries.to_vec()), expect.to_vec(),)
}
#[test]
fn example1() {
    check([[1, 2, 3], [2, 5, 7], [3, 5, 1]], [5, 6, 2], [5, 8, 1]);
}

#[test]
fn example2() {
    check([[5, 2, 1], [1, 1, 2]], [3], [0]);
}
