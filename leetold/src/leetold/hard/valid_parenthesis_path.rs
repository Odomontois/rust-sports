use std::collections::HashSet;

pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
    let m = grid.get(0).map_or(0, |x| x.len());
    let mut cur: Vec<HashSet<i32>> = vec![HashSet::new(); m];
    cur[0] = Some(0).into_iter().collect();
    for row in grid {
        let prev = cur;
        let pairs = row.into_iter().zip(prev);
        let elems = pairs.scan(HashSet::new(), |left, (c, up)| {
            let update = if c == '(' {
                |x| Some(x + 1)
            } else {
                |x| (x > 0).then(|| x - 1)
            };
            *left = left.union(&up).copied().filter_map(update).collect();
            Some(left.clone())
        });
        cur = elems.collect();
    }
    cur.last().into_iter().any(|nums| nums.contains(&0))
}

fn check(xs: &[&str]) -> bool {
    has_valid_path(xs.iter().map(|s| s.chars().collect()).collect())
}

#[test]
fn test1() {
    assert!(!check(&["()", "()"]))
}
