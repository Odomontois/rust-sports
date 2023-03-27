use std::cell::RefCell;

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = if let Some(v) = grid.get(0) { v.len() } else { return 0 };
    let cache = RefCell::new(vec![vec![-1; m]; n]);
    struct Iter<'a>(&'a dyn Fn(&Iter, usize, usize) -> i32);
    let f = move |iter: &Iter, i: usize, j| {
        let r = |i, j| iter.0(iter, i, j);
        match cache.borrow()[i][j] {
            -1 => {}
            num => return num,
        }
        let up = (i > 0).then(|| r(i - 1, j));
        let left = (j > 0).then(|| r(i, j - 1));
        let res = grid[i][j] + up.into_iter().chain(left).min().unwrap_or(0);
        cache.borrow_mut()[i][j] = res;
        res
    };
    let iter = Iter(&f);
    iter.0(&iter, n - 1, m - 1)
}
