pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    Walk::new(grid, k).result()
}

struct Walk {
    cache: Vec<Option<Option<u16>>>,
    grid: Vec<Vec<i32>>,
    n: usize,
    m: usize,
    k: usize,
}
impl Walk {
    fn cache(&mut self, i: i16, j: i16, k: i16) -> &mut Option<Option<u16>> {
        let (i, j, l) = (i as usize, j as usize, self.k + 1);
        &mut self.cache[i * l * self.m + j * l + k as usize]
    }
    fn new(grid: Vec<Vec<i32>>, k: i32) -> Self {
        let n = grid.len();
        let m = grid[0].len();
        let k = k as usize;
        let cache = vec![None; (k + 1) * m * n];
        Self { cache, n, m, grid, k }
    }
    fn calc(&mut self, i: i16, j: i16, k: i16) -> Option<u16> {
        if !((0..self.n as i16).contains(&i) && (0..self.m as i16).contains(&j) && k >= 0) {
            return None;
        };
        if (i, j) == (0, 0) {
            return Some(0);
        }
        let cache = self.cache(i, j, k);
        if let Some(res) = *cache {
            return res;
        }
        *cache = Some(None);
        for &[x, y] in &[[i - 1, j], [i + 1, j], [i, j + 1], [i, j - 1]] {
            if let Some(neighbor) = self.calc(x, y, k - self.grid[i as usize][j as usize] as i16) {
                if let Some(cur) = self.cache(i, j, k) {
                    *cur = Some(cur.map_or(neighbor + 1, |x| x.min(neighbor + 1)))
                }
            }
        }

        (*self.cache(i, j, k))?
    }
    fn result(&mut self) -> i32 {
        self.calc(self.n as i16 - 1, self.m as i16 - 1, self.k as i16)
            .map_or(-1, |x| x as i32)
    }
}

#[cfg(test)]
fn check<const N: usize>(v: &[[i32; N]], k: i32, exp: i32) {
    assert_eq!(shortest_path(v.iter().map(|v| v.to_vec()).collect(), k), exp);
}

#[test]
fn test1() {
    check(&[[0, 0, 0], [1, 1, 0], [0, 0, 0], [0, 1, 1], [0, 0, 0]], 1, 6);
}

#[test]
fn test2() {
    check(&[[0, 1, 1], [1, 1, 1], [1, 0, 0]], 1, -1);
}
