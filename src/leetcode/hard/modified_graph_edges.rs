pub fn modified_graph_edges(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32, target: i32) -> Vec<Vec<i32>> {
    Search::new(n as usize, edges, source, destination)
        .solve(target)
        .map(|edges| edges.into_iter().map(|(u, v, w)| vec![u as i32, v as i32, w]).collect())
        .unwrap_or_default()
}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    i32,
};

struct Search {
    graph: Vec<Vec<(usize, i32, bool)>>,
    source: usize,
    destination: usize,
    blank: Vec<[[usize; 2]; 2]>,
}

impl Search {
    pub fn new(n: usize, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> Self {
        let mut graph = vec![vec![]; n as usize];
        let mut blank = vec![];
        for edge in edges {
            match &edge[..] {
                &[u, v, w] => {
                    let (u, v) = (u as usize, v as usize);
                    let is_blank = w == -1;
                    graph[u].push((v, w, is_blank));
                    graph[v].push((u, w, is_blank));
                    if is_blank {
                        blank.push([[u, graph[u].len() - 1], [v, graph[v].len() - 1]]);
                    }
                }
                _ => panic!("invalid edge {:?}", edge),
            }
        }
        let source = source as usize;
        let destination = destination as usize;
        Search { graph, source, destination, blank }
    }

    fn shortest_path(&self) -> Option<(i64, Vec<(usize, usize)>)> {
        let mut best = vec![None; self.graph.len()];
        let mut q = BinaryHeap::new();
        q.push((Reverse(0i64), self.source, None));
        while let Some((Reverse(dist), cur, prev)) = q.pop() {
            if best[cur].is_some() {
                continue;
            }
            best[cur] = Some((dist, prev));
            if cur == self.destination {
                break;
            }
            for &(next, w, is_blank) in &self.graph[cur] {
                if best[next].is_none() {
                    q.push((Reverse(dist + w as i64), next, Some((cur, is_blank))));
                }
            }
        }
        let mut cur = self.destination;
        let mut blank_mid = vec![];
        while cur != self.source {
            let (prev, is_blank) = best[cur]?.1?;
            if is_blank {
                blank_mid.push((prev, cur));
            }
            cur = prev;
        }
        let best = best[self.destination]?.0;
        Some((best, blank_mid))
    }

    pub fn set_blanks(&mut self, value: i32) {
        self.blank.iter().for_each(|&[[i, ip], [j, jp]]| {
            self.graph[i][ip].1 = value;
            self.graph[j][jp].1 = value;
        });
    }

    pub fn solve(&mut self, target: i32) -> Option<Vec<(usize, usize, i32)>> {
        self.set_blanks(target + 1);

        let target = target as i64;

        if let Some((best, _)) = self.shortest_path() {
            if best < target {
                return None;
            }
            if best == target {
                return Some(self.collect_edges());
            }
        }
        self.set_blanks(1);

        let (best, blank_mid) = self.shortest_path()?;
        if best > target {
            return None;
        } else if best == target {
            return Some(self.collect_edges());
        }
        let seen: HashSet<_> = blank_mid.into_iter().collect();
        let (fu, fv) = seen.iter().next().copied()?;
        for &[[i, ip], [j, jp]] in &self.blank {
            let add = if !seen.contains(&(i, j)) && !seen.contains(&(j, i)) {
                Some(target)
            } else if (i, j) == (fu, fv) || (j, i) == (fu, fv) {
                Some(target - best)
            } else {
                None
            };
            if let Some(add) = add {
                self.graph[i][ip].1 += add as i32;
                self.graph[j][jp].1 += add as i32;
            }
        }

        Some(self.collect_edges())
    }

    fn collect_edges(&self) -> Vec<(usize, usize, i32)> {
        let mut edges = vec![];
        for (i, adj) in self.graph.iter().enumerate() {
            for &(j, w, _) in adj {
                if i < j {
                    edges.push((i, j, w));
                }
            }
        }
        edges
    }
}

#[cfg(test)]
fn check(n: i32, edges: &[[i32; 3]], source: i32, destination: i32, target: i32, expected: &[[i32; 3]]) {
    let mut result = modified_graph_edges(
        n,
        edges.iter().map(|a| a.to_vec()).collect(),
        source,
        destination,
        target,
    );
    result.sort_unstable();
    let mut expected = expected.to_owned();
    expected.iter_mut().for_each(|[x, y, _]| {
        (*x, *y) = (*x.min(y), *x.max(y));
    });
    expected.sort_unstable();
    assert_eq!(result, expected);
}

#[test]
fn example1() {
    check(
        5,
        &[[4, 1, -1], [2, 0, -1], [0, 3, -1], [4, 3, -1]],
        0,
        1,
        5,
        &[[4, 1, 1], [2, 0, 1], [0, 3, 3], [4, 3, 1]],
    )
}

#[test]
fn example2() {
    check(3, &[[0, 1, -1], [0, 2, 5]], 0, 2, 6, &[])
}
#[test]
fn example3() {
    check(
        4,
        &[[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, -1]],
        0,
        2,
        6,
        &[[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, 1]],
    )
}
