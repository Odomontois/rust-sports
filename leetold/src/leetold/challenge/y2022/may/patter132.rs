
fn find132pattern<A: Ord>(nums: impl AsRef<[A]>) -> bool {
    let mut maxs = Vec::new();
    let mut max_mid = None;
    nums.as_ref().iter().rev().any(|x| {
        let res = Some(x) < max_mid;
        while maxs.last().into_iter().copied().any(|y| y < x) {
            max_mid = max_mid.max(maxs.pop());
        }
        maxs.push(x);
        res
    })
}

#[test]
fn example1() {
    assert!(!find132pattern([1, 2, 3, 4]))
}

#[test]
fn example2() {
    assert!(find132pattern([3, 1, 4, 2]))
}

#[test]
fn example3() {
    assert!(find132pattern([-1, 3, 2, 0]))
}

#[test]
fn wa1() {
    assert!(find132pattern([3, 5, 0, 3, 4]))
}
