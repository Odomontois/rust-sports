use std::collections::HashMap;

pub fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
    let mut evs = HashMap::new();
    for v in segments {
        *evs.entry(v[0] as i64).or_insert(0) += v[2] as i64;
        *evs.entry(v[1] as i64).or_insert(0) -= v[2] as i64;
    }
    let (mut cc, mut res) = (0, vec![]);
    let mut evs: Vec<_> = evs.into_iter().collect();
    evs.sort();
    for v in evs.windows(2) {
        cc += v[0].1;
        if cc != 0 {
            res.push(vec![v[0].0, v[1].0, cc]);
        }
    }
    res
}

#[cfg(test)]
fn check(ss: &[[i32; 3]], exp: &[[i64; 3]]) {
    assert_eq!(
        exp.iter().map(|v| v.to_vec()).collect::<Vec<_>>(),
        split_painting(ss.iter().map(|v| v.to_vec()).collect())
    )
}

#[test]
fn test1() {
    check(
        &[[1, 4, 5], [1, 4, 7], [4, 7, 1], [4, 7, 11]],
        &[[1, 4, 12], [4, 7, 12]],
    );
}

#[test]
fn test2() {
    check(
        &[[1, 7, 9], [6, 8, 15], [8, 10, 7]],
        &[[1, 6, 9], [6, 7, 24], [7, 8, 15], [8, 10, 7]],
    );
}

#[test]
fn test3() {
    check(&[[1, 4, 5], [4, 7, 7], [1, 7, 9]], &[[1, 4, 14], [4, 7, 16]])
}

#[test]
fn test4() {
    check(
        &[
            [4, 16, 12],
            [9, 10, 15],
            [18, 19, 13],
            [3, 13, 20],
            [12, 16, 3],
            [2, 10, 10],
            [3, 11, 4],
            [13, 16, 6],
        ],
        &[
            [2, 3, 10],
            [3, 4, 34],
            [4, 9, 46],
            [9, 10, 61],
            [10, 11, 36],
            [11, 12, 32],
            [12, 13, 35],
            [13, 16, 21],
            [18, 19, 13],
        ],
    );
}
