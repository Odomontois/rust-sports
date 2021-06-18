fn printer_error(s: &str) -> String {
    format!(
        "{bad}/{total}",
        bad = s.chars().filter(|c| !('a'..='m').contains(c)).count(),
        total = s.len()
    )
}
