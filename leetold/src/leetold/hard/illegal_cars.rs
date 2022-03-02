// https://leetcode.com/problems/minimum-time-to-remove-all-cars-containing-illegal-goods/

pub fn minimum_time(s: impl AsRef<str>) -> i32 {
    let s = s.as_ref();
    let r2l = times(s.chars().rev()).collect::<Vec<_>>().into_iter().rev();
    times(s.chars()).zip(r2l).map(|(x, y)| x + y).min().unwrap_or(0)
}

fn times(c: impl Iterator<Item = char>) -> impl Iterator<Item = i32> {
    let update = |best: &mut i32, (i, c)| {
        if c == '1' {
            *best = (*best + 2).min(i as i32 + 1)
        }
        Some(*best)
    };
    Some(0).into_iter().chain(c.enumerate().scan(0, update))
}

#[test]
fn test1() {
    assert_eq!(5, minimum_time("1100101"))
}

#[test]
fn test2() {
    assert_eq!(2, minimum_time("0010"))
}
