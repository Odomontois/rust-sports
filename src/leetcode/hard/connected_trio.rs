use std::{collections::HashSet, usize};

pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let edges: Vec<[usize; 2]> = edges
        .into_iter()
        .map(|v| v.into_iter().map(|x| x as usize - 1).collect::<Vec<_>>())
        .map(|v| [v[0], v[1]])
        .collect();
    let mut adj = vec![HashSet::new(); n as usize];

    for &[a, b] in &edges {
        adj[a].insert(b);
        adj[b].insert(a);
    }
    edges
        .iter()
        .flat_map(|&[a, b]| {
            let x = adj[a].len() + adj[b].len();
            let ad = &adj;
            adj[a].intersection(&adj[b]).map(move |&c| (x + ad[c].len() - 6) as i32)
        })
        .min()
        .unwrap_or(-1)
}

#[test]
fn test() {
    fn check(xs: &[[i32; 2]], exp: i32) {
        let edges: Vec<_> = xs.iter().map(|x| x.to_vec()).collect();
        let n = edges.iter().flatten().copied().max().unwrap_or(0);
        assert_eq!(min_trio_degree(n, edges), exp)
    }
    check(&[[1, 2], [1, 3], [3, 2], [4, 1], [5, 2], [3, 6]], 3);
    check(&[[1, 3], [4, 1], [4, 3], [2, 5], [5, 6], [6, 7], [7, 5], [2, 6]], 0);
}
