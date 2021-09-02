pub fn min_flips_mono_incr(s: String) -> i32 {
    (|t: usize| {
        s.chars()
            .map(|c| (c == '0') as usize)
            .enumerate()
            .fold((0, s.len() - t), |(z, m), (i, cz)| (z + cz, m.min(i + t - 2 * z)))
            .1 as i32
    })(s.chars().filter(|&s| s == '0').count())
}
