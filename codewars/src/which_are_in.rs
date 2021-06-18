fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut res: Vec<_> = arr_a
        .iter()
        .filter(|&s| arr_b.iter().any(|s2| s2.contains(s)))
        .map(|s| s.to_string())
        .collect();

    res.sort();
    res.dedup();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            in_array(
                &["xyz", "live", "strong"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["live", "strong"]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );

        assert_eq!(
            in_array(
                &["tarp", "mice", "bull"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            [] as [&str; 0]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );
    }
}
