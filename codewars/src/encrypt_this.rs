use itertools::Itertools;

#[allow(unstable_name_collisions)]
fn encrypt_this(text: &str) -> String {
    text.split(" ")
        .map(|s| {
            let s1 = s.as_bytes();
            let tail = if s.len() <= 2 {
                s[1..].to_string()
            } else {
                format!(
                    "{}{}{}",
                    s1[s.len() - 1] as char,
                    String::from_utf8_lossy(&s1[2..s.len() - 1]),
                    s1[1] as char
                )
            };
            format!("{}{}", s1[0], tail)
        })
        .intersperse(" ".to_string())
        .collect()
}
