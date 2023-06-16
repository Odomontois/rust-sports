use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(Clone, Copy)]
struct Move<'a, A> {
    grid: &'a Vec<Vec<A>>,
    ix: usize,
    hor: bool,
}

impl<'a, A: Copy> Move<'a, A> {
    fn elems(&self) -> impl Iterator<Item = A> + '_ {
        let &Move { grid, ix, hor } = self;
        let n = grid.len();
        (0..n).map(move |i| if hor { grid[ix][i] } else { grid[i][ix] })
    }
}

impl<A: PartialEq + Copy> PartialEq for Move<'_, A> {
    fn eq(&self, other: &Self) -> bool {
        self.elems().eq(other.elems())
    }
}
impl<A: Eq + Copy> Eq for Move<'_, A> {}

impl<A: Hash + Copy> Hash for Move<'_, A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elems().for_each(|x| x.hash(state))
    }
}

fn counts<A: Hash + Eq>(xs: impl Iterator<Item = A>) -> HashMap<A, i32> {
    let mut counts = HashMap::new();
    xs.for_each(|x| *counts.entry(x).or_insert(0) += 1);
    counts
}

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let grid = &grid;
    let hors = counts((0..n).map(|ix| Move { grid, ix, hor: true }));
    let verts = counts((0..n).map(|ix| Move { grid, ix, hor: false }));
    let common = hors.keys().filter(|k| verts.contains_key(k));
    common.map(|k| hors[k] * verts[k]).sum()
}
