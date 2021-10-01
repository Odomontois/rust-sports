pub fn max_length(arr: Vec<String>) -> i32 {
    let mut v = vec![0];
    for c in arr.iter().flat_map(|s| to_bitset(s)) {
        let mut xs = v.iter().filter(|&&z| z & c == 0).map(|&z| z | c).collect();
        v.append(&mut xs);
    }
    v.into_iter().map(|x| x.count_ones() as i32).max().unwrap_or(0)
}

fn to_bitset(s: &str) -> Option<u32> {
    let mut res = 0;
    for c in s.chars() {
        let c = 1 << (c as u8 - 'a' as u8);
        if res & c != 0 {
            return None;
        }
        res |= c
    }
    Some(res)
}
