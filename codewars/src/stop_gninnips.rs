use itertools::Itertools;

#[allow(unstable_name_collisions)]
fn spin_words(words: &str) -> String {
    words
        .split(" ")
        .map(|w| {
            if w.len() < 5 {
                w.to_string()
            } else {
                w.chars().rev().collect()
            }
        })
        .intersperse(" ".to_string())
        .collect()
}
