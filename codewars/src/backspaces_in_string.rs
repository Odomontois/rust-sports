fn clean_string(s: &str) -> String {
    let mut res = vec![];
    for c in s.chars() {
        if c == '#' {
            res.pop();
        } else {
            res.push(c);
        }
    }
    res.into_iter().collect()
}
