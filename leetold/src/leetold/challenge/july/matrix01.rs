use std::collections::VecDeque;

fn inside(res: &[Vec<i32>], x: usize, y: usize) -> Option<(usize, usize)> {
    if (1..=res.len()).contains(&x) && (1..=res[0].len()).contains(&y) && res[x - 1][y - 1] == -1 {
        Some((x - 1, y - 1))
    } else {
        None
    }
}

fn neighbors(i: usize, j: usize, res: &[Vec<i32>]) -> Vec<(usize, usize)> {
    vec![(i + 2, j + 1), (i, j + 1), (i + 1, j + 2), (i + 1, j)]
        .into_iter()
        .filter_map(move |(x, y)| inside(res, x, y))
        .collect()
}

pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let zeros = mat.iter().enumerate().flat_map(|(i, v): (usize, &Vec<i32>)| {
        v.iter()
            .enumerate()
            .filter_map(move |(j, &x)| Some((i, j)).filter(|_| x == 0))
    });
    let mut res = vec![vec![-1; mat[0].len()]; mat.len()];
    for (i, j) in zeros.clone() {
        res[i][j] = 0
    }
    let mut q: VecDeque<_> = zeros.map(|p| (p, 0)).collect();

    while let Some(((i, j), p)) = q.pop_front() {
        for (x, y) in neighbors(i, j, &res) {
            res[x][y] = p + 1;
            q.push_back(((x, y), p + 1));
        }
    }

    res
}
