use std::collections::BTreeSet;

pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
    let n = n as usize;
    let mut dist = vec![None; n];
    dist[0] = Some(0);
    let edges: Vec<_> = edges
        .into_iter()
        .map(|e| (e[0] as usize, e[1] as usize, e[2] + 1))
        .collect();
    let mut graph = vec![vec![]; n];
    for &(i, j, v) in &edges {
        graph[i].push((j, v));
        graph[j].push((i, v));
    }
    let mut q = BTreeSet::new();
    q.insert((0, 0));
    while let Some(&(d, i)) = q.iter().next() {
        q.remove(&(d, i));
        for (j, v) in graph[i].drain(..) {
            let w = v + d;
            match dist[j] {
                None => {
                    q.insert((w, j));
                    dist[j] = Some(w);
                }
                Some(u) if u > w => {
                    q.remove(&(u, j));
                    q.insert((w, j));
                    dist[j] = Some(w);
                }
                Some(_) => {}
            }
        }
    }
    let vd = dist.iter().flatten().filter(|&&d| d <= max_moves).count() as i32;
    let rem = |i: usize| (max_moves - dist[i].unwrap_or(max_moves)).max(0);
    let ed: i32 = edges.iter().map(|&(i, j, v)| (rem(i) + rem(j)).min(v - 1)).sum();
    vd + ed
}

#[cfg(test)]
fn check(exp: i32, n: usize, max_moves: i32, edges: &[[i32; 3]]) {
    assert_eq!(
        exp,
        reachable_nodes(edges.iter().map(|v| v.to_vec()).collect(), max_moves, n as i32)
    )
}

#[test]
fn test1() {
    check(13, 3, 6, &[[0, 1, 10], [0, 2, 1], [1, 2, 2]])
}
