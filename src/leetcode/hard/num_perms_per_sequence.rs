pub fn num_perms_di_sequence(s: String) -> i32 {
    s.chars()
        .fold(vec![1], next_sequence)
        .into_iter()
        .fold(0, |x, y| (x + y) % MOD)
}

const MOD: i32 = 1_000_000_007;

fn next_sequence(prev: Vec<i32>, c: char) -> Vec<i32> {
    let n = prev.len();
    let mut next = vec![0; n + 1];
    for (i, x) in prev.into_iter().enumerate() {
        let rng = if c == 'D' { 0..=i } else { i + 1..=n };
        for j in rng {
            next[j] = (next[j] + x) % MOD;
        }
    }
    next
}
