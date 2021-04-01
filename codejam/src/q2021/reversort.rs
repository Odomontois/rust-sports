use std::io::{self, prelude::*, Stdin};

pub fn main() {
    reversort_main()
}

pub fn reversort_main() {
    let stdin = io::stdin();
    for (i, t) in tests(&stdin).enumerate() {
        println!("Case #{}: {:?}", i + 1, reversort(t))
    }
}

pub fn reversort(mut xs: Vec<i32>) -> i32 {
    let mut q = 0;
    for i in 0..xs.len() - 1 {
        let j = (i..xs.len()).min_by_key(|&j| xs[j]).unwrap();
        q += j - i + 1;
        xs[i..=j].reverse()
    }
    q as i32
}

fn tests(stdin: &Stdin) -> impl Iterator<Item = Vec<i32>> + '_ {
    let mut reader = stdin.lock().lines().map(Result::unwrap);
    let count: u32 = reader.next().unwrap().parse().unwrap();
    (0..count).map(move |_| {
        reader.next().unwrap();
        reader
            .next()
            .unwrap()
            .split(" ")
            .flat_map(|s| s.parse())
            .collect()
    })
}
