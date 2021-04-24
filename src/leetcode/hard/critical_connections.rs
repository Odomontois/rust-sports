pub fn critical_connections(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let m = edges.len();

    let mut adj = vec![Vec::new(); n];
    for (i, v) in edges.into_iter().enumerate() {
        let (x, y) = (v[0] as usize, v[1] as usize);
        adj[x].push([y, i]);
        adj[y].push([x, i]);
    }
    let mut link = vec![0u32; n];
    let mut used = vec![false; m];
    let mut on_stack = vec![false; n];
    let mut seen = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();
    let mut iter_stack = Vec::new();
    let mut idx = vec![0u32; n];
    let mut next = 1u32;
    for i in 0..n {
        let mut child_link = None;
        if seen[i] {
            continue;
        }
        iter_stack.push([i, 0]);
        while let Some([v, k]) = iter_stack.pop() {
            if k == 0 {
                seen[v] = true;
                stack.push(v);
                on_stack[v] = true;
                idx[v] = next;
                link[v] = next;
                next += 1;
            } else if let Some(ln) = child_link {
                link[v] = link[v].min(ln);
            }
            if k < adj[v].len() {
                iter_stack.push([v, k + 1]);
                let [w, j] = adj[v][k];
                if used[j] {
                    child_link = None;
                    continue;
                }
                used[j] = true;
                if !seen[w] {
                    iter_stack.push([w, 0])
                } else if on_stack[w] {
                    child_link = Some(idx[w]);
                } else {
                    child_link = None;
                }
            } else if link[v] == idx[v] {
                while let Some(w) = stack.pop() {
                    on_stack[w] = false;
                    link[w] = link[v];
                    if v == w {
                        break;
                    }
                }
            } else {
                child_link = Some(link[v])
            }
        }
    }
    let link = &link;
    adj.iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter()
                .map(|[j, _]| *j)
                .filter(move |&j| i < j && link[i] != link[j])
                .map(move |j| vec![i as i32, j as i32])
        })
        .collect()
}

#[test]
fn test() {
    fn sorted(mut xs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for v in &mut xs {
            v.sort()
        }
        xs.sort();
        xs
    }
    fn check(xs: &[[i32; 2]], n: i32, exp: &[[i32; 2]]) {
        assert_eq!(
            sorted(critical_connections(n, xs.iter().map(|v| v.to_vec()).collect())),
            sorted(exp.iter().map(|v| v.to_vec()).collect())
        )
    }

    // check(&[[0, 1], [1, 2], [2, 0], [1, 3]], 4, &[[1, 3]]);
    // check(&[], 1, &[]);
    // check(
    //     &[
    //         [0, 1],
    //         [1, 2],
    //         [2, 3],
    //         [3, 0],
    //         [1, 3],
    //         [0, 2],
    //         [3, 4],
    //         [4, 5],
    //         [5, 6],
    //         [6, 7],
    //         [7, 4],
    //     ],
    //     8,
    //     &[[3, 4]],
    // );
    // check(&[[2, 4], [0, 2], [4, 1], [1, 3], [3, 0], [0, 4], [0, 1]], 5, &[]);
    check(&[[0, 1], [1, 2], [2, 3], [3, 4], [4, 2], [2, 5], [0, 5]], 6, &[]);
}
