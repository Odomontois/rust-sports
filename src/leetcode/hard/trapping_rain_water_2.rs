pub fn trap_rain_water<R: AsRef<[i32]>, M: AsRef<[R]>>(height_map: M) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap, iter::from_fn};
    let n = height_map.as_ref().len();
    let m = height_map.as_ref()[0].as_ref().len();
    let mut level = vec![1_000_000_000; m * n];
    let height = move |x: usize, y| height_map.as_ref()[x].as_ref()[y];
    let init_hor = || (0..n).flat_map(|i| [(i, 0), (i, m - 1)]);
    let init_ver = || (1..m - 1).flat_map(|j| [(0, j), (n - 1, j)]);
    let init = || init_hor().chain(init_ver()).map(|(x, y)| (height(x, y), (x, y)));
    init().for_each(|(h, (x, y))| {
        level[x * m + y] = h;
    });
    let mut q = BinaryHeap::from_iter(init().map(|(h, p)| (Reverse(h), p)));
    let ixs = |x, y| [(x, y + 1), (x + 2, y + 1), (x + 1, y + 2), (x + 1, y)].into_iter();
    let good = |x: usize, y: usize| x > 0 && x <= n && y > 0 && y <= m;
    let neighbors = move |x, y| ixs(x, y).filter_map(move |(i, j)| good(i, j).then(|| (i - 1, j - 1)));
    from_fn(|| {
        let (Reverse(h), (x, y)) = q.pop()?;
        if level[x * m + y] < h {
            return Some(0);
        }
        for (i, j) in neighbors(x, y) {
            let nh = h.max(height(i, j));
            if level[i * m + j] <= nh {
                continue;
            }
            level[i * m + j] = nh;
            q.push((Reverse(nh), (i, j)));
        }
        Some(h - height(x, y))
    })
    .sum()
}

#[test]
fn example1() {
    assert_eq!(
        4,
        trap_rain_water([[1, 4, 3, 1, 3, 2], [3, 2, 1, 3, 2, 4], [2, 3, 3, 2, 3, 1]])
    )
}

#[test]
fn example2() {
    assert_eq!(
        10,
        trap_rain_water([
            [3, 3, 3, 3, 3],
            [3, 2, 2, 2, 3],
            [3, 2, 1, 2, 3],
            [3, 2, 2, 2, 3],
            [3, 3, 3, 3, 3]
        ])
    )
}
