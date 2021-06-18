fn wave(s: &str) -> Vec<String> {
    s.chars()
        .enumerate()
        .filter_map(|(k, x)| {
            if x.is_alphanumeric() {
                Some(
                    s.chars()
                        .enumerate()
                        .map(move |(i, c)| {
                            if i == k {
                                c.to_uppercase().collect()
                            } else {
                                c.to_string()
                            }
                        })
                        .collect(),
                )
            } else {
                None
            }
        })
        .collect()
}
