use std::collections::HashSet;

pub fn fib(n: i32) -> i32 {
    let mut fs = vec![0, 1];
    for i in 2..=(n as usize) { fs.push(fs[i - 1] + fs[i - 2]) }
    fs[n as usize]
}

pub fn distribute_candies(candies: Vec<i32>) -> i32 {
    let s: HashSet<i32> = candies.iter().cloned().collect();
    s.len().min(candies.len() / 2) as i32
}