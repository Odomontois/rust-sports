use std::collections::{HashSet, VecDeque};

use data::neighbors::Neigbors;

pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let neigh = Neigbors::matrix(&grid);

    let mut colour = 2;
    let mut sizes = vec![];
    for (i, j) in neigh.walk() {
        if grid[i][j] != 1 {
            continue;
        }
        grid[i][j] = colour;
        let mut q: VecDeque<_> = vec![(i, j)].into_iter().collect();
        let mut size = 1;
        while let Some((x, y)) = q.pop_front() {
            for (u, v) in neigh.neighbors(x, y) {
                if grid[u][v] == 1 {
                    grid[u][v] = colour;
                    size += 1;
                    q.push_back((u, v))
                }
            }
        }
        colour += 1;
        sizes.push(size);
    }

    neigh
        .walk()
        .filter(|&(i, j)| grid[i][j] == 0)
        .map(|(i, j)| {
            neigh
                .neighbors(i, j)
                .map(|(x, y)| grid[x][y])
                .collect::<HashSet<_>>()
                .into_iter()
                .map(|c| if c > 1 { sizes[c as usize - 2] } else { 0 })
                .sum()
        })
        .max()
        .unwrap_or((neigh.hsize * neigh.vsize - 1) as i32)
        + 1
}

fn check<const N: usize>(exp: i32, grid: &[[i32; N]]) {
    assert_eq!(exp, largest_island(grid.iter().map(|v| v.to_vec()).collect()))
}

#[test]
fn test1() {
    check(3, &[[1, 0], [0, 1]])
}

#[test]
fn test2() {
    check(4, &[[1, 1], [1, 0]])
}

#[test]
fn test3() {
    check(4, &[[1, 1], [1, 1]])
}
