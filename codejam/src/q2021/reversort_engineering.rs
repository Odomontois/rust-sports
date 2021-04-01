use std::io::{self, prelude::*, Stdin};

pub fn main() {
    rev_ing_main()
}

pub fn rev_ing_main() {
    let stdin = io::stdin();
    for (i, (n, c)) in tests(&stdin).enumerate() {
        let res: String = match reversort_ing(n, c) {
            Some(v) => v.iter().map(i32::to_string).collect::<Vec<_>>().join(" "),
            None => "IMPOSSIBLE".to_string(),
        };
        println!("Case #{}: {}", i + 1, res)
    }
}

fn good(n: usize, c: usize) -> bool {
    c + 1 >= n && c < (n * (n + 1)) / 2
}

fn insert(k: usize, mut arr: Vec<i32>) -> Vec<i32> {
    for x in &mut arr {
        *x += 1
    }
    arr.insert(0, 1);
    arr[0..k].reverse();
    arr
}

fn reversort_ing(n: usize, c: usize) -> Option<Vec<i32>> {
    if !good(n, c) {
        return None;
    }
    if n == 1 {
        return Some(vec![1]);
    }
    let k = (1..=n).find(|&k| good(n - 1, c - k))?;
    let lower = reversort_ing(n - 1, c - k)?;
    Some(insert(k, lower))
}

fn tests(stdin: &Stdin) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut reader = stdin.lock().lines().map(Result::unwrap);
    let count: u32 = reader.next().unwrap().parse().unwrap();
    (0..count).filter_map(move |_| {
        let s = reader.next()?;
        let mut parts: _ = s.split(" ").flat_map(str::parse);
        Some((parts.next()?, parts.next()?))
    })
}


