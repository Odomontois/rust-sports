pub fn min_insertions(s: String) -> i32 {
    let s = s.as_bytes();
    let mut all = vec![0u16; s.len() * 3];
    let (mut prev, mid) = all.split_at_mut(s.len() + 1);
    let (mut cur, mut next) = mid.split_at_mut(s.len());
    for i in 1..s.len() {
        for j in 0..s.len() - i {
            next[j] = if s[j] == s[j + i] {
                prev[j + 1]
            } else {
                1 + cur[j].min(cur[j + 1])
            }
        }
        std::mem::swap(&mut prev, &mut cur);
        std::mem::swap(&mut cur, &mut next);
    }
    cur[0] as i32
}
