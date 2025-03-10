use std::io::{self, prelude::*, Stdin};

pub fn main() {
    moon_main()
}

fn moon_umbrellas(cj: i32, jc: i32, cjq: Vec<CJQ>) -> i32 {
    const ALOT: i32 = 1000000000;
    cjq.into_iter()
        .fold([0, 0], |[c, j], x| match x {
            C => [c.min(j + jc), ALOT],
            J => [ALOT, j.min(c + cj)],
            Q => [c.min(j + jc), j.min(c + cj)],
        })
        .iter()
        .min()
        .copied()
        .unwrap_or(0)
}

pub fn moon_main() {
    let stdin = io::stdin();
    for (i, (x, y, cjq)) in tests(&stdin).enumerate() {
        println!("Case #{}: {:?}", i + 1, moon_umbrellas(x, y, cjq))
    }
}

enum CJQ {
    C,
    J,
    Q,
}
use CJQ::*;
fn tests(stdin: &Stdin) -> impl Iterator<Item = (i32, i32, Vec<CJQ>)> + '_ {
    let mut reader = stdin.lock().lines().map(Result::unwrap);
    let count: u32 = reader.next().unwrap().parse().unwrap();
    (0..count).filter_map(move |_| {
        let s = reader.next()?;
        let mut elems = s.split(" ");
        let x = elems.next()?.parse().ok()?;
        let y = elems.next()?.parse().ok()?;
        let cjq = elems.next()?.chars().map(|c| match c {
            'C' => C,
            'J' => J,
            _ => Q,
        });
        Some((x, y, cjq.collect()))
    })
}
