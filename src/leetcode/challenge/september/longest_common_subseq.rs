pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    lcs(text1.as_bytes(), text2.as_bytes()) as i32
}

fn lcs<A: Eq>(a: &[A], b: &[A]) -> usize {
    (0..b.len()).fold(vec![0; a.len() + 1], |pl, j| {
        let f = |i, p: usize| if a[i] == b[j] { pl[i] + 1 } else { p.max(pl[i + 1]) };
        scan(0..a.len(), 0, f).collect()
    })[a.len()]
}

fn scan<'a, A, B: Clone + 'a>(
    it: impl Iterator<Item = A> + 'a,
    b: B,
    f: impl Fn(A, B) -> B + 'a,
) -> impl Iterator<Item = B> + 'a {
    it.scan(Some(b), move |b, a| {
        *b = Some(f(a, b.take().unwrap()));
        b.clone()
    })
}
