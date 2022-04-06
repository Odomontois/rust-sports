use std::{collections::BTreeSet, convert::TryInto};

#[derive(Clone, Copy, Debug)]
enum Cost {
    None,
    Best(u32),
    Two(u32, u32),
}

pub fn second_minimum<A, V, I>(n: i32, edges: I, time: i32, change: i32) -> i32
where
    A: TryInto<usize>,
    V: IntoIterator<Item = A>,
    I: IntoIterator<Item = V>,
{
    let n = n as usize;
    let mut adj = vec![vec![] as Vec<usize>; n];
    let mut best = vec![Cost::None; n];
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
    let mut q = BTreeSet::new();

    q.insert((0, 0));

    while let Some(&(cost, idx)) = q.iter().next() {
        q.remove(&(cost, idx));
        let cost = cost + 1;
        for &next in &adj[idx] {
            let (switch, put, remove) = match best[next] {
                Cost::None => (Cost::Best(cost), true, None),
                Cost::Best(b) if b != cost => (Cost::Two(b.min(cost), b.max(cost)), true, None),
                Cost::Two(b, s) if cost < s && b != cost => (Cost::Two(b.min(cost), b.max(cost)), true, Some(s)),
                c => (c, false, None),
            };
            best[next] = switch;
            if put {
                q.insert((cost, next));
            }
            for cost in remove {
                q.remove(&(cost, next));
            }
        }
    }

    let steps = match best[n - 1] {
        Cost::Two(_, s) => s,
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
