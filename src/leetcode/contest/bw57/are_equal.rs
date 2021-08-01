pub fn are_occurrences_equal(s: String) -> bool {
    let mut counts = vec![0; 26];
    for c in s.bytes() {
        counts[(c - 'a' as u8) as usize] += 1;
    }
    let c = counts.iter().copied().filter(|&x| x > 0).next().unwrap_or(0);
    counts.into_iter().all(|x| x == 0 || x == c)
}