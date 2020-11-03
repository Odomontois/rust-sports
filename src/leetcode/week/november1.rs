pub fn max_power(s: String) -> i32 {
    s.chars().scan((1i32, '\0'), |(count, prev), c| {
        if *prev == c { *count += 1 } else { *count = 1 }
        *prev = c;
        Some(*count)
    }).max().unwrap_or(0)
}

