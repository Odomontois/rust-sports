use std::iter::{from_fn};

fn fibs() -> impl Iterator<Item = u64> {
    let (mut cur, mut prev) = (1, 1);
    from_fn(move || {
        let pprev = prev;
        prev = cur;
        cur = cur + pprev;
        Some(prev)
    })
}

#[test]
fn check() {
    println!("{:?}", fibs().take(30).collect::<Vec<_>>())
}
