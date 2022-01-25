pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
    let grid = grid
        .into_iter()
        .map(|v| v.into_iter().map(|x| x == 1).collect())
        .collect();
    let stamp_height = stamp_height as usize;
    Grid {
        grid: grid,
        stamp_height: stamp_height as usize,
        stamp_width: stamp_width as usize,
    }
    .possible()
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<bool>>,
    stamp_height: usize,
    stamp_width: usize,
}

fn distances(good: impl IntoIterator<Item = bool>, offset: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut dist = offset;
    for nice in good {
        dist = if nice { dist + 1 } else { 0 };
        res.push(dist)
    }
    res
}

impl Grid {
    fn horizontal(&self, offset: usize) -> Vec<Vec<usize>> {
        self.grid
            .iter()
            .map(|v| distances(v.iter().map(|&x| !x), offset))
            .collect()
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
    fn width(&self) -> usize {
        self.grid.iter().next().map(|v| v.len()).unwrap_or(0)
    }

    fn conditional(&self, hor_offset: usize, ver_offset: usize) -> Grid {
        let hor = self.horizontal(hor_offset);
        let vacc = (0..self.width())
            .map(|j| distances((0..self.height()).map(|i| hor[i][j] >= self.stamp_width), ver_offset))
            .collect::<Vec<_>>();

        let stampable = |i: usize, j: usize| {
            vacc.get(j + self.stamp_width - 1 - hor_offset)
                .and_then(|v| v.get(i + self.stamp_height - 1 - ver_offset))
                .into_iter()
                .any(|&t| t >= self.stamp_height)
        };
        let grid = (0..self.height())
            .map(|i| (0..self.width()).map(|j| stampable(i, j)).collect())
            .collect();
        Self { grid, ..*self }
    }

    fn stamps(&self) -> Grid {
        self.conditional(0, 0)
    }

    fn unstamped(&self) -> Grid {
        self.conditional(self.stamp_width - 1, self.stamp_height - 1)
    }

    fn possible(&self) -> bool {
        let unst = self.stamps().unstamped();
        unst.grid
            .iter()
            .zip(&self.grid)
            .flat_map(|(unst_row, occup_row)| unst_row.iter().zip(occup_row))
            .all(|(&unstamped, &occupied)| occupied || (!unstamped))
    }
}

#[test]
fn lolones() {
    assert!(check(&["...", "...", "XXX", "...", "..."], 3, 2))
}

#[test]
fn loltwoes() {
    assert!(check(
        &["xxxxxxx", "x..xxxx", "x..x..x", "xxxx..x", "..xxx..", "..xxx..", "xxxxxxx"],
        2,
        2
    ))
}

#[test]
fn loltwoesow() {
    assert!(!check(
        &["xxxxxxx", "x..xxxx", "x..x..x", "xxxxx.x", "..xxx..", "..xxx..", "xxxxxxx"],
        2,
        2
    ))
}

fn check(ss: &[&str], width: i32, height: i32) -> bool {
    possible_to_stamp(
        ss.iter()
            .map(|&s| s.chars().map(|c| if "Xx1".contains(c) { 1 } else { 0 }).collect())
            .collect(),
        height,
        width,
    )
}
