

use State::*;

#[derive(Debug)]
enum State {
    Start,
    Climb(usize),
    Decline(usize),
}

#[allow(dead_code)]
pub fn longest_mountain(a: Vec<i32>) -> i32 {
    a.into_iter().scan((Start, None::<i32>), |(state, prev), i| {
        let inc = prev.iter().any(|p| *p < i);
        let dec = prev.iter().any(|p| *p > i);
        *prev = Some(i);
        *state = match *state {
            Start => if inc { Climb(2) } else { Start },
            Climb(n) => if inc { Climb(n + 1) } else if dec { Decline(n + 1) } else { Start },
            Decline(n) => if dec { Decline(n + 1) } else if inc { Climb(2) } else { Start }
        };
        Some(if let Decline(n) = *state { n } else { 0 })
    }).max().unwrap_or(0) as i32
}

#[test]
fn mountain() {
    println!("{:?}", longest_mountain(vec![2, 2, 2]))
}

#[allow(dead_code)]
pub fn mirror_reflection(p: i32, q: i32) -> i32 {
    let g = gcd(p, q);
    if p / g % 2 == 0 { 2 } else if q / g % 2 == 0 { 0 } else { 1 }
}

use std::ops::Rem;

fn gcd<A: Rem<Output=A> + From<u8> + Ord + Clone>(a: A, b: A) -> A {
    if b > a { return gcd(b, a); };
    let r = a % b.clone();
    if r == 0.into() { b } else { gcd(b, r) }
}

#[test]
fn mirror() {
    assert_eq!(mirror_reflection(120, 57), 2)
}