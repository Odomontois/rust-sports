use std::iter::once;
use itertools::Itertools;

fn solution(s: &str) -> Vec<String> {
    s.chars()
        .chain(once('_'))
        .tuples()
        .map(|(a, b)| format!("{}{}", a, b))
        .collect()
}
