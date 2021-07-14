use std::{collections::HashMap, ops::BitOrAssign};

#[derive(Debug, Clone, Copy)]
struct Rect {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}
impl Rect {
    fn new(i: usize, j: usize) -> Self {
        Self {
            up: i,
            down: i,
            left: j,
            right: j,
        }
    }
    fn indices(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (self.up..=self.down).flat_map(move |i| (self.left..=self.right).map(move |j| (i, j)))
    }

    fn fits(&self, xs: &Vec<Vec<i32>>, col: i32) -> bool {
        self.indices().all(|(i, j)| xs[i][j] == col || xs[i][j] == -1)
    }

    fn unprint(&self, xs: &mut Vec<Vec<i32>>) {
        self.indices().for_each(|(i, j)| xs[i][j] = -1)
    }
}

impl BitOrAssign for Rect {
    fn bitor_assign(&mut self, rhs: Self) {
        self.up = self.up.min(rhs.up);
        self.down = self.down.max(rhs.down);
        self.left = self.left.min(rhs.left);
        self.right = self.right.max(rhs.right)
    }
}
pub fn is_printable(mut grid: Vec<Vec<i32>>) -> bool {
    let mut colors = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            let rect = Rect::new(i, j);
            colors.entry(col).and_modify(|r| *r |= rect).or_insert(rect);
        }
    }

    while !colors.is_empty() {
        if let Some((&color, rect)) = colors.iter().find(|(&color, rect)| rect.fits(&grid, color)) {
            rect.unprint(&mut grid);
            colors.remove(&color);
        } else {
            return false;
        }
    }

    true
}
