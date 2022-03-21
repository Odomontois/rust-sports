pub fn orderly_queue(s: String, k: i32) -> String {
    if k == 1 {
        best_shift(s)
    } else {
        sorted_chars(s)
    }
}

fn sorted_chars(s: String) -> String {
    let mut v = s.as_bytes().to_vec();
    v.sort();
    String::from_utf8(v).unwrap_or_default()
}

fn best_shift(s: String) -> String {
    let shift = |i| s[i..].chars().chain(s[..i].chars());
    let oi = (0..s.len()).min_by(|&i, &j| shift(i).cmp(shift(j)));
    shift(oi.unwrap_or(0)).collect()
}
