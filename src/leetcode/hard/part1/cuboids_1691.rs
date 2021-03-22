use std::collections::{HashSet, VecDeque};
use std::iter::once;

pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
    let cuboids: Vec<_> = cuboids.iter().enumerate().flat_map(
        |(i, v)| rotations([v[0], v[1], v[2]]).map(move |c| (i, c))).collect();
    let n = cuboids.len();
    let edges: Vec<Vec<_>> = cuboids.iter().map(|&(i, [l1, w1, h1])|
        (0..n).filter(|&k| {
            let (j, [l2, w2, h2]) = cuboids[k];
            l2 <= l1 && w2 <= w1 && h2 <= h1 && i != j && (i < j || l2 < l1 || w2 < w1 || h2 < h1)
        }).collect()
    ).chain(once((0..n).collect())).collect();

    let mut inv = vec![HashSet::new(); n];
    for (i, e) in edges.iter().enumerate() {
        for &j in e { inv[j].insert(i); }
    }
    let mut q: VecDeque<_> = once((n, 0)).collect();
    let mut best = vec![0; n];
    while let Some((i, h)) = q.pop_front() {
        for &j in &edges[i] {
            best[j] = best[j].max(h + cuboids[j].1[2]);
            inv[j].remove(&i);
            if inv[j].is_empty() {
                q.push_back((j, best[j]))
            }
        }
    }
    best.into_iter().max().unwrap_or(0)
}

const ROTATIONS: [[usize; 3]; 6] = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];

type Cuboid = [i32; 3];

fn rotations(c: Cuboid) -> impl Iterator<Item=Cuboid> {
    ROTATIONS.iter().map(move |&[i, j, k]| [c[i], c[j], c[k]])
}

#[test]
fn test() {
    fn check<'a, A>(xs: A, exp: i32) where A: IntoIterator<Item=&'a [i32; 3]> {
        let vs = xs.into_iter().map(|v| v.iter().copied().collect()).collect();
        assert_eq!(max_height(vs), exp)
    }
    check(&[[50, 45, 20], [95, 37, 53], [45, 23, 12]], 190);
    check(&[[38, 25, 45], [76, 35, 3]], 76);
    check(&[[7, 11, 17], [7, 17, 11], [11, 7, 17], [11, 17, 7], [17, 7, 11], [17, 11, 7]], 102);
}




