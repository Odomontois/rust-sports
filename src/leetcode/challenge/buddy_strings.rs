fn eq_prefix<A: Eq>(s1: impl IntoIterator<Item = A>, s2: impl IntoIterator<Item = A>) -> usize {
    s1.into_iter().zip(s2).take_while(|(c1, c2)| c1 == c2).count()
}

fn nonunique<A, I>(s: impl IntoIterator<Item = A>, code: impl Fn(A) -> I) -> bool
where
    u32: std::ops::Shl<I, Output = u32>,
{
    let mut seen = 0u32;
    s.into_iter().any(|c| {
        let x = 1 << code(c);
        seen & x != 0 || {
            seen |= x;
            false
        }
    })
}

pub fn buddy_strings(s: String, goal: String) -> bool {
    let (s, goal) = (s.as_bytes(), goal.as_bytes());
    let n = s.len();
    if n != goal.len() {
        return false;
    }
    let i = eq_prefix(s, goal);
    if i == s.len() {
        return nonunique(s, |c| *c as u8 - 'a' as u8);
    }

    let j = n - eq_prefix(s.iter().rev(), goal.iter().rev()) - 1;
    if i + n - j == s.len() {
        return false;
    }

    s[i] == goal[j] && s[j] == goal[i] && s[i + 1..j] == goal[i + 1..j]
}
