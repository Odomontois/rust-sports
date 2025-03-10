#[derive(Clone, Copy, Debug)]
enum Choice {
    L,
    R,
    B,
}
pub fn shortest_common_supersequence(str_left: String, str_right: String) -> String {
    let n = str_left.len();
    let m = str_right.len();
    let mut d = vec![(Choice::B, 0u16); (n + 1) * (m + 1)];
    let idx = |i: usize, j: usize| i * (n + 1) + j;
    for j in 1..=n {
        d[idx(0, j)] = (Choice::L, j as u16 + 1);
    }
    for (i, cr) in str_right.bytes().rev().enumerate() {
        d[idx(i + 1, 0)] = (Choice::R, i as u16 + 1);
        for (j, cl) in str_left.bytes().rev().enumerate() {
            d[idx(i + 1, j + 1)] = if cl == cr {
                (Choice::B, d[idx(i, j)].1 + 1)
            } else if d[idx(i + 1, j)].1 < d[idx(i, j + 1)].1 {
                (Choice::L, d[idx(i + 1, j)].1 + 1)
            } else {
                (Choice::R, d[idx(i, j + 1)].1 + 1)
            }
        }
    }
    let (sl, sr) = (str_left.as_bytes(), str_right.as_bytes());
    String::from_utf8(
        (0..d[idx(m, n)].1)
            .scan((m, n), |(i, j), _| {
                let (c, ni, nj) = match d[idx(*i, *j)].0 {
                    Choice::B => (sl[n - *j], *i - 1, *j - 1),
                    Choice::L => (sl[n - *j], *i, *j - 1),
                    Choice::R => (sr[m - *i], *i - 1, *j),
                };
                (*i, *j) = (ni, nj);
                Some(c)
            })
            .collect(),
    )
    .unwrap_or_default()
}

#[test]
fn example1() {
    assert_eq!(
        "cabac",
        shortest_common_supersequence("abac".to_string(), "cab".to_string())
    );
}

#[test]
fn example2() {
    assert_eq!(
        "aaaaaaaa",
        shortest_common_supersequence("aaaaaaaa".to_string(), "aaaaaaaa".to_string())
    );
}
#[test]
fn test1(){
    assert_eq!(
        "tyu",
        shortest_common_supersequence("ty".to_string(), "yu".to_string())
    )
}
