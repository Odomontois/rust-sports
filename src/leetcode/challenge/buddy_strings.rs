fn eq_prefix<A: Eq>(s1: impl IntoIterator<Item = A>, s2: impl IntoIterator<Item = A>) -> usize {
    s1.into_iter().zip(s2).take_while(|(c1, c2)| c1 == c2).count()
}

pub fn buddy_strings(s: String, goal: String) -> bool {
    let (s, goal) = (s.as_bytes(), goal.as_bytes());
    let n = s.len();
    if n != goal.len() {
        return false;
    }
    let l1 = eq_prefix(s, goal);
    if l1 == s.len() {
        let mut seen = 0;
        return s.iter().any(|c| {
            let x = 1 << (*c - 'a' as u8);
            seen & x != 0 || {
                seen |= x;
                false
            }
        });
    }

    let l2 = eq_prefix(s.iter().rev(), goal.iter().rev());
    if l1 + l2 + 1 == s.len() {
        return false;
    }

    s[l1] == goal[n - l2 - 1] && s[n - l2 - 1] == goal[l1]
}
