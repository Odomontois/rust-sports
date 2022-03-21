use std::{collections::HashMap, iter::repeat, str::Bytes};

const P: u64 = 6_876_537_510_269_753_087;

pub fn longest_dup_substring(s: String) -> String {
    let (mut l, mut u, mut h) = (0, 0, s.len() + 1);
    while h > l + 1 {
        let m = (h + l) / 2;
        if let Some([v, _]) = repeated(s.bytes(), m).filter(|&[u, v]| s[u..u + m] == s[v..v + m]) {
            u = v;
            l = m;
        } else {
            h = m
        }
    }
    s[u..u + l].to_string()
}

pub fn repeated(xs: Bytes, window: usize) -> Option<[usize; 2]> {
    let mut hs = HashMap::<u64, usize>::new();
    for (i, h) in hashes(xs.map(|u| u as u64), window).enumerate() {
        if let Some(j) = hs.insert(h, i) {
            return Some([j, i]);
        }
    }
    None
}

pub fn hashes(xs: impl Iterator<Item = u64> + Clone, window: usize) -> impl Iterator<Item = u64> {
    let (mp, _) = P.overflowing_pow(window as u32);
    let tail = repeat(0).take(window).chain(xs.clone());
    xs.zip(tail)
        .scan(0u64, move |h, (c, t)| {
            *h = h
                .overflowing_mul(P)
                .0
                .overflowing_add(c)
                .0
                .overflowing_sub(t.overflowing_mul(mp).0)
                .0;
            Some(*h)
        })
        .skip(window - 1)
}

#[test]
fn test1() {
    assert_eq!(longest_dup_substring("banana".to_string()), "ana")
}
