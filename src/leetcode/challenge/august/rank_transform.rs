use std::{collections::HashMap, hash::Hash, vec};

pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    Ranker::of_matrix(matrix).rank_matrix()
}

#[derive(Debug, Clone)]
enum Elem<I> {
    Data { elems: Vec<I>, rank: i32 },
    Link(I),
}

struct Ranker<A> {
    matrix: Vec<Vec<A>>,
    elems: Vec<Vec<Elem<(usize, usize)>>>,
    row_rank: Vec<i32>,
    col_rank: Vec<i32>,
}

impl<A> Ranker<A>
where
    A: Copy + Ord + Hash,
{
    fn rank_matrix(&mut self) -> Vec<Vec<i32>> {
        for i in 0..self.elems.len() {
            self.union_line((0..self.col_rank.len()).map(|j| (i, j)))
        }
        for j in 0..self.col_rank.len() {
            self.union_line((0..self.elems.len()).map(|i| (i, j)))
        }
        let elems = &self.elems;

        let mut groups: Vec<_> = self
            .matrix
            .iter()
            .enumerate()
            .flat_map(|(i, v)| {
                v.iter().enumerate().filter_map(move |(j, &d)| match elems[i][j] {
                    Elem::Data { .. } => Some((d, i, j)),
                    _ => None,
                })
            })
            .collect();
        groups.sort_by_key(|(k, _, _)| *k);
        for (_, i, j) in groups {
            self.handle_group(i, j)
        }

        (0..self.matrix.len())
            .map(|i| {
                (0..self.col_rank.len())
                    .map(|j| {
                        let (ij, _) = self.root((i, j));
                        *self.rank_of(ij)
                    })
                    .collect()
            })
            .collect()
    }

    fn rank_of(&mut self, (i, j): (usize, usize)) -> &mut i32 {
        match &mut self.elems[i][j] {
            Elem::Data { rank, .. } => rank,
            _ => panic!("not a root"),
        }
    }

    fn handle_group(&mut self, i: usize, j: usize) {
        let els = match &self.elems[i][j] {
            Elem::Data { elems, .. } => elems,
            _ => return,
        };

        let its = || els.iter().copied().chain(vec![(i, j)]);

        let new_rank = its()
            .flat_map(|(i, j)| vec![self.row_rank[i], self.col_rank[j]])
            .max()
            .unwrap_or(0)
            + 1;

        for (i, j) in its() {
            self.row_rank[i] = new_rank;
            self.col_rank[j] = new_rank;
        }

        *self.rank_of((i, j)) = new_rank
    }

    fn union_line(&mut self, its: impl Iterator<Item = (usize, usize)>) {
        let mut hm = HashMap::<A, (usize, usize)>::new();
        for (i, j) in its {
            let v = self.matrix[i][j];
            if let Some(&p) = hm.get(&v) {
                self.union(p, (i, j))
            } else {
                hm.insert(v, (i, j));
            }
        }
    }

    pub fn of_matrix(matrix: Vec<Vec<A>>) -> Self {
        let data = Elem::Data { elems: vec![], rank: 0 };
        let n = matrix.len();
        let m = matrix.iter().next().map(|v| v.len()).unwrap_or(0);
        Self {
            matrix,
            row_rank: vec![0; n],
            col_rank: vec![0; m],
            elems: vec![vec![data; m]; n],
        }
    }

    fn root(&mut self, (row, col): (usize, usize)) -> ((usize, usize), usize) {
        match self.elems[row][col] {
            Elem::Data { ref elems, .. } => ((row, col), elems.len()),
            Elem::Link(p) => {
                let (p, size) = self.root(p);
                self.elems[row][col] = Elem::Link(p);
                (p, size)
            }
        }
    }

    fn link_root(&mut self, (r1, c1): (usize, usize), (r2, c2): (usize, usize)) {
        let mut d2 = Elem::Link((r1, c1));
        std::mem::swap(&mut d2, &mut self.elems[r2][c2]);
        if let Elem::Data { elems: mut es, .. } = d2 {
            if let Elem::Data { elems, .. } = &mut self.elems[r1][c1] {
                elems.push((r2, c2));
                elems.append(&mut es);
            }
        }
    }

    fn union(&mut self, p1: (usize, usize), p2: (usize, usize)) {
        let (r1, s1) = self.root(p1);
        let (r2, s2) = self.root(p2);
        if r1 == r2 {
            return;
        } else if s1 > s2 {
            self.link_root(r1, r2);
        } else {
            self.link_root(r2, r1)
        }
    }
}

#[cfg(test)]
fn check<const N: usize>(inp: &[[i32; N]], exp: &[[i32; N]]) {
    assert_eq!(
        matrix_rank_transform(inp.iter().map(|v| v.to_vec()).collect()),
        exp.iter().map(|v| v.to_vec()).collect::<Vec<_>>()
    )
}
#[test]
fn test1() {
    check(&[[1, 2], [3, 4]], &[[1, 2], [2, 3]])
}

#[test]
fn test2() {
    check(&[[7, 7], [7, 7]], &[[1, 1], [1, 1]])
}

#[test]
fn test3() {
    check(
        &[[20, -21, 14], [-19, 4, 19], [22, -47, 24], [-19, 4, 19]],
        &[[4, 2, 3], [1, 3, 4], [5, 1, 6], [1, 3, 4]],
    );
}

#[test]
fn test4() {
    check(&[[7, 3, 6], [1, 4, 5], [9, 8, 2]], &[[5, 1, 4], [1, 2, 3], [6, 3, 1]])
}
