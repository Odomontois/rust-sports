use std::collections::BTreeSet;

pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
    let n = n as usize;
    let direct = edges.iter().map(|v| (v[0] as usize, v[1] as usize, v[2] as i64));
    let graph = Graph::new(n, direct.clone());
    let inv = Graph::new(n, direct.map(|(x, y, c)| (y, x, c)));
    let from1 = graph.costs(src1 as usize);
    let from2 = graph.costs(src2 as usize);
    let to = inv.costs(dest as usize);
    let cost = |i| from1(i).zip(from2(i)).zip(to(i)).map(|((a, b), c)| a + b + c);
    (0..n).filter_map(cost).min().unwrap_or(-1)
}

#[derive(Clone, Debug)]
struct Graph {
    size: usize,
    adjacent: Vec<Vec<(usize, i64)>>,
}

impl Graph {
    fn new(size: usize, edges: impl IntoIterator<Item = (usize, usize, i64)>) -> Self {
        let mut adjacent = vec![vec![]; size];
        for (x, y, cost) in edges {
            adjacent[x].push((y, cost))
        }
        Self { size, adjacent }
    }
    fn costs(&self, start: usize) -> impl Fn(usize) -> Option<i64> {
        let mut costs = vec![None; self.size];
        let mut best = BTreeSet::<(i64, usize)>::new();
        costs[start] = Some(0);
        best.insert((0, start));
        while let Some(&(p, x)) = best.iter().next() {
            best.remove(&(p, x));
            for &(y, c) in &self.adjacent[x] {
                let new_cost = c + p;
                if costs[y].iter().any(|&z| z <= new_cost) {
                    continue;
                }
                if let Some(prev_cost) = costs[y] {
                    best.remove(&(prev_cost, y));
                }
                costs[y] = Some(new_cost);
                best.insert((new_cost, y));
            }
        }
        move |i| costs[i]
    }
}

#[cfg(test)]
fn check(exp: i64, n: i32, edges: &[[i32; 3]], src1: i32, src2: i32, dest: i32) {
    assert_eq!(
        exp,
        minimum_weight(n, edges.iter().map(|v| v.to_vec()).collect(), src1, src2, dest)
    )
}

#[test]
fn test1() {
    check(
        9,
        6,
        &[
            [0, 2, 2],
            [0, 5, 6],
            [1, 0, 3],
            [1, 4, 5],
            [2, 1, 1],
            [2, 3, 3],
            [2, 3, 4],
            [3, 4, 2],
            [4, 5, 1],
        ],
        0,
        1,
        5,
    )
}
