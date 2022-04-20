use std::{collections::VecDeque, convert::TryInto};

pub fn second_minimum<A, V, I>(n: i32, edges: I, time: i32, change: i32) -> i32
where
    A: TryInto<usize>,
    V: IntoIterator<Item = A>,
    I: IntoIterator<Item = V>,
{
    let n = n as usize;
    let mut adj = vec![vec![] as Vec<usize>; n];
    let mut best = vec![None; n];
    let to_szarr = |xs: V| {
        xs.into_iter()
            .map(opt_into)
            .collect::<Option<Vec<_>>>()
            .and_then(opt_into)
            .map(|v: [usize; 2]| v.map(|x| x - 1))
    };
    for [x, y] in edges.into_iter().filter_map(to_szarr) {
        adj[x].push(y);
        adj[y].push(x);
    }
    let mut q = VecDeque::new();

    q.push_back((0, 0));

    while let Some((cost, idx)) = q.pop_front() {
        let cost = cost + 1;
        for &next in &adj[idx] {
            let (switch, put) = match best[next] {
                None => (Some((cost, None)), true),
                Some((b, None)) if b < cost => (Some((b, Some(cost))), true),
                c => (c, false),
            };
            best[next] = switch;
            if put {
                q.push_back((cost, next));
            }
        }
    }

    let steps = match best[n - 1] {
        Some((_, Some(s))) => s,
        _ => return -1,
    };

    (0..steps).fold(0, |x, _| x + ((x / change) % 2) * (change - (x % change)) + time)
}

fn opt_into<A: TryInto<B>, B>(x: A) -> Option<B> {
    x.try_into().ok()
}

#[test]
fn test1() {
    assert_eq!(13, second_minimum(5, [[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]], 3, 5))
}

#[test]
fn test2() {
    assert_eq!(11, second_minimum(2, [[1, 2]], 3, 2))
}