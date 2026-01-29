pub fn min_cost_bin_search(grid: Vec<impl AsRef<[i32]>>, k: i32) -> i32 {
    use std::{cmp::Ordering, collections::BTreeSet};

    #[derive(Eq, PartialEq, Ord, PartialOrd)]
    struct Item {
        cost: i32,
        used: usize,
        x: usize,
        y: usize,
    }

    struct Q<T: AsRef<[i32]>> {
        q: BTreeSet<Item>,
        grid: Vec<T>,
        seen: Vec<i32>,
        m: usize,
        n: usize,
        k: usize,
    }

    impl<T: AsRef<[i32]>> Q<T> {
        fn new(grid: Vec<T>, k: usize) -> Self {
            let n = grid.len();
            let m = grid[0].as_ref().len();
            let seen = vec![i32::MAX; n * m * k];
            Q { q: BTreeSet::from_iter([Item { cost: 0, x: 0, y: 0, used: 0 }]), grid, seen, m, n, k }
        }
        fn put(&mut self, mut item: Item, add_cost: bool) {
            if item.x >= self.n || item.y >= self.m {
                return;
            }
            if add_cost {
                item.cost += self.grid[item.x].as_ref()[item.y];
            }
            let seen = self.seen(&item);
            if *seen > item.cost {
                *seen = item.cost;
                self.q.insert(item);
            }
        }

        fn val(&self, x: usize, y: usize) -> i32 {
            self.grid[x].as_ref()[y]
        }

        fn seen(&mut self, item: &Item) -> &mut i32 {
            &mut self.seen[item.x * self.m * self.k + item.y * self.k + item.used]
        }
    }

    let n = grid.len();
    let m = grid[0].as_ref().len();
    let k = k as usize + 1;
    let mut searcher = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.as_ref().iter().enumerate().map(move |(j, x)| (i, j, *x)))
        .collect::<Vec<_>>();
    searcher.sort_unstable_by_key(|(_, _, c)| *c);
    let search = |v: i32| {
        let i = searcher
            .binary_search_by(|(_, _, w)| {
                let o = w.cmp(&v);
                if o == Ordering::Equal { Ordering::Less } else { o }
            })
            .unwrap_err();
        &searcher[..i]
    };

    let mut q = Q::new(grid, k);
    let mut best = i32::MAX;

    while let Some(ref item @ Item { cost, x, y, used }) = q.q.pop_first() {
        if cost > *q.seen(item) {
            continue;
        }
        if (x, y) == (n - 1, m - 1) {
            best = best.min(cost);
        }
        if used < k - 1 {
            for &(x, y, _) in search(q.val(x, y)) {
                q.put(Item { x, y, used: used + 1, cost }, false)
            }
        }
        q.put(Item { x: x + 1, y, used, cost }, true);
        q.put(Item { x, y: y + 1, used, cost }, true);
    }

    best
}

fn min_cost(grid: Vec<impl AsRef<[i32]>>, k: i32) -> i32 {
    let n = grid.len();
    let m = grid[0].as_ref().len();
    let max_grid = grid.iter().flat_map(|r| r.as_ref()).copied().max().unwrap_or(0) as usize;
    let mut mins = vec![i32::MAX; max_grid + 1];
    let mut costs = vec![i32::MAX; n * m];
    let idx = |i: usize, j: usize| i * m + j;
    let cells = || {
        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| row.as_ref().iter().enumerate().map(move |(j, &cell)| (i, j, cell)))
    };
    costs[0] = 0;
    for _ in 0..=k {
        for (i, j, cell) in cells() {
            let mut v = costs[idx(i, j)];
            v = mins[cell as usize].min(v);
            if i > 0 {
                v = v.min(costs[idx(i - 1, j)] + cell);
            }
            if j > 0 {
                v = v.min(costs[idx(i, j - 1)] + cell);
            }
            costs[idx(i, j)] = v;
        }

        for (i, j, cell) in cells() {
            let m = &mut mins[cell as usize];
            *m = costs[idx(i, j)].min(*m);
        }
        for c in (0..max_grid).rev() {
            mins[c] = mins[c].min(mins[c + 1]);
        }
    }
    costs[idx(n - 1, m - 1)]
}

#[test]
fn example1() {
    assert_eq!(7, min_cost(vec![[1, 3, 3], [2, 5, 4], [4, 3, 5]], 2))
}

#[test]
fn example2() {
    assert_eq!(9, min_cost(vec![[1, 2], [2, 3], [3, 4]], 1))
}

#[rustfmt::skip]
#[test]
fn test1() {
    assert_eq!(21, min_cost(
        vec![
            [0, 1, 2], 
            [1000, 1000, 1000], 
            [2, 3, 4],
            [1000, 1000, 1000],
            [4, 5, 6],
        ], 2));

    assert_eq!(21, min_cost(
        vec![
            [0, 1000, 2, 1000, 4], 
            [1, 1000, 3, 1000, 5], 
            [2, 1000, 4, 1000, 6], 
        ], 2));
}

#[test]
fn wa1() {
    assert_eq!(49, min_cost(vec![[22, 12], [23, 49]], 2))
}
