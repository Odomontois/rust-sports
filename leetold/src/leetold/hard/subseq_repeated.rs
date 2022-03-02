use std::{
    cmp::Reverse,
    iter::{once, repeat},
};

pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
    let mut counts = [0; 26];
    let k = k as usize;
    let code = |c: char| (c as u8 - 'a' as u8) as usize;
    for c in s.chars().map(code) {
        counts[c] += 1
    }
    let symb = |(i, &cnt)| repeat(i as u8 + 'a' as u8).take(cnt / k);
    let symbs: Vec<_> = counts.iter().enumerate().flat_map(symb).collect();
    let mut seqs = combs(symbs);

    seqs.sort_by_key(|v| Reverse(v.len()));

    for seq in seqs {
        if subseq(s.bytes(), seq.iter().copied().cycle().take(seq.len() * k)) {
            return String::from_utf8(seq).unwrap();
        }
    }

    "".to_string()
}

fn subseq<A: Eq>(seq: impl IntoIterator<Item = A>, sub: impl Iterator<Item = A>) -> bool {
    let mut check = sub.peekable();
    for a in seq {
        match check.peek() {
            None => return true,
            Some(b) if &a == b => drop(check.next()),
            _ => {}
        }
    }
    check.next().is_none()
}

fn combs_iter<A: Copy>(t: &mut Vec<A>) -> Vec<Vec<A>> {
    let gen = (0..t.len()).flat_map(|i| {
        let u = t.remove(i);
        let mut vs = combs_iter(t);
        t.insert(i, u);
        for v in &mut vs {
            v.push(u)
        }
        vs
    });

    once(vec![]).chain(gen).collect()
}

fn combs<A: Copy>(mut t: Vec<A>) -> Vec<Vec<A>> {
    let mut res = combs_iter(&mut t);
    for v in &mut res {
        v.reverse();
    }
    res.reverse();
    res
}

#[test]
fn combs_test() {
    println!("{:?}", combs(vec![1, 2, 3]));
}

#[cfg(test)]
fn check(s: &str, k: i32, exp: &str) {
    assert_eq!(exp, longest_subsequence_repeated_k(s.to_string(), k))
}

#[test]
fn test1() {
    check("letsleetcode", 2, "let")
}


