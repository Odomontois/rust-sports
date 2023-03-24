use std::{collections::VecDeque, convert::TryInto};

pub fn min_reorder<V: TryInto<[i32; 2]>>(n: i32, connections: impl IntoIterator<Item = V>) -> i32 {
    let n = n as usize;
    let connections = connections
        .into_iter()
        .filter_map(|v| v.try_into().ok())
        .map(|v| v.map(|v| v as usize));
    let mut adj = vec![vec![]; n];
    for [x, y] in connections {
        adj[x].push((y, true));
        adj[y].push((x, false));
    }
    let mut visited = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    let mut res = 0;
    while let Some(x) = q.pop_front() {
        visited[x] = true;
        for &(y, forward) in &adj[x] {
            if visited[y] {
                continue;
            }
            q.push_back(y);
            if forward {
                res += 1;
            }
        }
    }
    res
}

#[test]
fn test() {
    min_reorder::<Vec<i32>>(1, vec![]);
    assert_eq!(3, min_reorder(6, [[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]]));
}
