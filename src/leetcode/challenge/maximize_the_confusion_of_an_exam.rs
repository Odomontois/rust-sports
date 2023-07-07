use std::iter::from_fn;

pub fn max_consecutive_answers<S: std::ops::Deref<Target = str>>(answer_key: S, k: i32) -> i32 {
    let run = |c| solve(answer_key.bytes(), c, k);
    run(b'F').chain(run(b'T')).max().unwrap_or(0)
}

fn solve<'a, X: Eq + Copy + 'a>(
    bs: impl IntoIterator<Item = X> + Clone + 'a,
    key: X,
    mut lim: i32,
) -> impl Iterator<Item = i32> + 'a {
    let mut streak = 0;
    let mut back = bs.clone().into_iter();
    let mut front = bs.into_iter();
    from_fn(move || {
        let c = back.next()?;
        streak += 1;
        if c != key {
            lim -= 1;
        }
        while lim < 0 {
            streak -= 1;
            if front.next() != Some(key) {
                lim += 1
            }
        }
        Some(streak)
    })
}

#[test]
fn example1() {
    assert_eq!(4, max_consecutive_answers("TTFF", 2))
}

#[test]
fn example2() {
    assert_eq!(3, max_consecutive_answers("TFFT", 1))
}

#[test]
fn example3() {
    assert_eq!(5, max_consecutive_answers("TTFTTFTT", 1))
}
