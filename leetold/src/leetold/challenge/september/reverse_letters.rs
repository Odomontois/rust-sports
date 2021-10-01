pub fn reverse_only_letters(s: String) -> String {
    let mut letters = s.chars().filter(|c| c.is_alphabetic()).rev();
    let next = |c: char| if c.is_alphabetic() { letters.next() } else { Some(c) };
    s.chars().filter_map(next).collect()
}
