use std::mem::swap;

pub fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let edges: Vec<_> = edges.into_iter().map(|v| [v[0] as usize, v[1] as usize]).collect();
    let mut best = vec![[None::<(i32, usize)>; 3]; scores.len()];
    for &[x, y] in &edges {
        update_max(&mut best[x], Some((scores[y], y)));
        update_max(&mut best[y], Some((scores[x], x)));
    }
    let variants = |x: usize, y: usize| best[x].iter().flatten().filter(move |(_, i)| *i != y);
    let edge_best = |[x, y]: [usize; 2]| {
        let xy = scores[x] + scores[y];
        variants(x, y)
            .flat_map(move |(u, a)| variants(y, x).filter_map(move |(v, b)| (a != b).then(|| u + v + xy)))
            .max()
    };
    edges.into_iter().filter_map(edge_best).max().unwrap_or(-1)
}

fn update_max<'a, A: Ord + 'a>(arr: impl IntoIterator<Item = &'a mut A>, mut val: A) {
    for a in arr.into_iter() {
        if &val > a {
            swap(a, &mut val)
        }
    }
}
