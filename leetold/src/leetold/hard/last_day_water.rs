use std::collections::HashSet;

type Point = [usize; 2];

const CLOSE: &[Point] = &[[2, 1], [0, 1], [1, 2], [1, 0]];
const FAR: &[Point] = &[[2, 1], [0, 1], [1, 2], [1, 0], [2, 2], [0, 0], [0, 2], [2, 0]];

pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    let arr = |v: Vec<i32>| [(v[0] - 1) as usize, (v[1] - 1) as usize];
    let [r, c] = [row as usize, col as usize];
    if row <= col {
        cross([r, c], cells.into_iter().map(arr), FAR) as i32
    } else {
        row * col - 1 - cross([c, r], cells.into_iter().rev().map(arr).map(|[x, y]| [y, x]), CLOSE) as i32
    }
}

fn cross([row, col]: Point, cells: impl Iterator<Item = Point>, nei: &[[usize; 2]]) -> usize {
    let mut ds = DS(vec![(true, 1); row * col]);
    let mut cur = vec![vec![None::<usize>; col]; row];
    for (d, [i, j]) in cells.enumerate() {
        cur[i][j] = Some(d);
        let mut try_merge = |x: usize, y: usize| {
            if x == 0 || y == 0 || x > row || y > col {
                return;
            }
            if let Some(f) = cur[x - 1][y - 1] {
                ds.union(d, f)
            }
        };
        for &[dx, dy] in nei {
            try_merge(i + dx, j + dy)
        }
        let left: HashSet<_> = (0..row).flat_map(|x| cur[x][0]).map(|i| ds.root(i)).collect();
        let moat = (0..row)
            .flat_map(|x| cur[x][col - 1])
            .map(|i| ds.root(i))
            .any(|k| left.contains(&k));
        if moat {
            return d;
        }
    }
    row * col
}

struct DS(Vec<(bool, usize)>);

impl DS {
    pub fn root(&mut self, i: usize) -> usize {
        let (root, x) = self.0[i];
        if root {
            return i;
        }
        let p = self.root(x);
        self.0[i].1 = p;
        return p;
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let i = self.root(i);
        let j = self.root(j);
        if i == j {
            return;
        }
        if self.0[i].1 < self.0[j].1 {
            self.0[j].1 = self.0[i].1 + self.0[j].1;
            self.0[i] = (false, j);
        } else {
            self.0[i].1 = self.0[i].1 + self.0[j].1;
            self.0[j] = (false, i);
        }
    }
}

#[cfg(test)]
fn check(exp: i32, row: i32, col: i32, cells: &[[i32; 2]]) {
    assert_eq!(
        exp,
        latest_day_to_cross(row, col, cells.iter().map(|s| s.to_vec()).collect())
    )
}

#[test]
fn test1() {
    check(2, 2, 2, &[[1, 1], [2, 1], [1, 2], [2, 2]])
}

#[test]
fn test2() {
    check(1, 2, 2, &[[1, 1], [1, 2], [2, 1], [2, 2]])
}

#[test]
fn test3() {
    check(
        3,
        3,
        3,
        &[[1, 2], [2, 1], [3, 3], [2, 2], [1, 1], [1, 3], [2, 3], [3, 2], [3, 1]],
    )
}
