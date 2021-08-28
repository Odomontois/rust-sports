use std::collections::HashMap;

pub fn find_lu_slength(strs: Vec<String>) -> i32 {
    let mut counter = HashMap::new();
    for s in strs {
        for ss in subseqs(s) {
            *counter.entry(ss).or_insert(0) += 1
        }
    }
    counter
        .into_iter()
        .filter(|(_, l)| *l == 1)
        .map(|(s, _)| s.len() as i32)
        .max()
        .unwrap_or(-1)
}

fn subseqs(str: String) -> impl Iterator<Item = String> {
    (0..2usize.pow(str.len() as u32)).map(move |bs| {
        str.chars()
            .enumerate()
            .filter_map(|(i, c)| if bs & (1 << i) == 0 { None } else { Some(c) })
            .collect()
    })
}
