use std::cmp::Ordering;

const GOOD: &[u64] = &[1, 2, 3, 3, 4, 10];
const BAD: &[u64] = &[1, 2, 2, 2, 3, 5, 10];
const GOOD_WIN: &str = "Good triumphs over Evil";
const BAD_WIN: &str = "Evil eradicates all trace of Good";
const TIE: &str = "No victor on this battle field";

fn good_vs_evil(good: &str, evil: &str) -> String {
    format!(
        "Battle Result: {}",
        match points(good, GOOD).cmp(&points(evil, BAD)) {
            Ordering::Less => BAD_WIN,
            Ordering::Equal => TIE,
            Ordering::Greater => GOOD_WIN,
        },
    )
}

fn points(s: &str, worth: &[u64]) -> u64 {
    s.split(" ")
        .zip(worth.iter())
        .map(|(p, &w)| p.parse().unwrap_or(0) * w)
        .sum()
}
