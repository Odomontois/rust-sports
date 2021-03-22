pub fn trap(height: Vec<i32>) -> i32 {
    let back: Vec<_> = max_height(height.iter().rev()).collect();
    height.iter().zip(max_height(height.iter()).zip(back.into_iter().rev())).map(
        |(&h, (l, r))| (l.min(r) - h).max(0)
    ).sum()
}

fn max_height<'a>(heights: impl Iterator<Item=&'a i32> + 'a) -> impl Iterator<Item=i32> + 'a {
    heights.scan(0, |m, &h| {
        *m = (*m).max(h);
        Some(*m)
    })
}

#[test]
fn trap_test() {
    fn check(h: &[i32], exp: i32) { assert_eq!(trap(h.iter().copied().collect()), exp) }
    check(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6)
}