pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    let mut counter = vec![0; 1 << 26];
    for w in &words {
        counter[mask(w, 0) as usize] += 1;
    }
    puzzles
        .iter()
        .map(|p| buckets(p).map(|m| counter[m as usize]).sum())
        .collect()
}

fn place(c: char) -> u32 {
    1 << (c as u8 - 'a' as u8)
}

fn mask(s: &str, drop: u32) -> u32 {
    let cs = s.chars().rev().enumerate();
    cs.fold(0, |m, (i, c)| if drop & (1 << i) == 0 { m | place(c) } else { m })
}

fn buckets(p: &str) -> impl Iterator<Item = u32> + '_ {
    (0..1 << 6).map(move |drop| mask(&p, drop))
}
