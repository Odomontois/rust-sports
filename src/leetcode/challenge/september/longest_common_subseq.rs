pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    lcs(text1.as_bytes(), text2.as_bytes()) as i32
}

fn lcs<A: Eq>(a: &[A], b: &[A]) -> usize {
    if a.len() > b.len() {
        return lcs(b, a);
    }
    (0..b.len()).fold(vec![0; a.len() + 1], |pl, j| {
        let mut cl = vec![0; a.len() + 1];
        for i in 0..a.len() {
            cl[i + 1] = if a[i] == b[j] { pl[i] + 1 } else { cl[i].max(pl[i + 1]) };
        }
        cl
    })[a.len()]
}
