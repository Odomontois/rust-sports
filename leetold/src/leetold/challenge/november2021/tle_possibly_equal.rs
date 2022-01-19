use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::{HashMap, HashSet},
    hash::Hash,
    iter::once,
};

pub fn possibly_equals(s1: String, s2: String) -> bool {
    let vs1 = variants(&s1);
    let vs2 = variants(&s2);
    for u in common(&vs1, &vs2) {
        for v1 in &vs1[&u] {
            for v2 in &vs2[&u] {
                if v1.suits(v2) {
                    return true;
                }
            }
        }
    }
    false
}

fn common<'a, A: Copy + Eq + Hash + 'a, B, C>(v1: &'a HashMap<A, B>, v2: &'a HashMap<A, C>) -> Vec<A> {
    let keys1: HashSet<_> = v1.keys().copied().collect();
    let keys2 = v2.keys().copied().collect();
    keys1.intersection(&keys2).copied().collect()
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
struct Variant {
    len: usize,
    letters: Vec<(usize, char)>,
}

impl Variant {
    fn add_len(&self, l: usize) -> Self {
        Self {
            len: self.len + l,
            ..self.clone()
        }
    }
    fn add_char(&mut self, c: char) {
        self.letters.push((self.len, c));
        self.len += 1;
    }
    fn suits(&self, v: &Variant) -> bool {
        if self.len != v.len {
            return false;
        }
        let mut i = self.letters.iter().peekable();
        let mut j = v.letters.iter().peekable();
        while let (Some((l1, c1)), Some((l2, c2))) = (i.peek(), j.peek()) {
            match l1.cmp(l2) {
                Less => {
                    i.next();
                }
                Greater => {
                    j.next();
                }
                Equal if c1 == c2 => {
                    i.next();
                    j.next();
                }
                Equal => return false,
            }
        }
        true
    }
}

fn variants<'a>(s: &'a str) -> HashMap<usize, Vec<Variant>> {
    let mut res = HashMap::new();
    for v in variants_it(s, Variant::default()) {
        res.entry(v.len).or_insert(Vec::default()).push(v);
    }
    res
}

fn variants_it<'a>(s: &'a str, mut acc: Variant) -> Box<dyn Iterator<Item = Variant> + 'a> {
    if s.is_empty() {
        return Box::new(once(acc));
    }

    let dpref = s.chars().take_while(|c| c.is_digit(10)).count();
    if dpref > 0 {
        let shifts = counts(&s[..dpref]).into_iter();

        let it = shifts.flat_map(move |u| variants_it(&s[dpref..], acc.add_len(u)));
        return Box::new(it);
    }

    let c = s.chars().next().unwrap();
    acc.add_char(c);
    variants_it(&s[1..], acc)
}

fn counts(digits: &str) -> Vec<usize> {
    let sub = |i: usize, j: usize| digits[i..j].parse::<usize>().unwrap();
    let mut res = match digits.len() {
        3 => vec![
            sub(0, 1) + sub(1, 2) + sub(2, 3),
            sub(0, 2) + sub(2, 3),
            sub(0, 1) + sub(1, 3),
            sub(0, 3),
        ],
        2 => vec![sub(0, 1) + sub(1, 2), sub(0, 2)],
        _ => vec![sub(0, 1)],
    };
    res.dedup();
    res
}

#[cfg(test)]
fn check(s1: &str, s2: &str, res: bool) {
    assert_eq!(possibly_equals(s1.to_string(), s2.to_string()), res)
}

#[test]
fn test1() {
    check("i18n", "internationalization", true)
}
#[test]
fn test2() {
    check("l123e", "44", true);
}

#[test]
fn test3() {
    check("a5b", "b5b", false);
}

#[test]
fn test4() {
    check("a5b", "a5c", false);
}

#[test]
fn test5() {
    check("a11a11", "a11b11", true)
}

#[test]
fn test6(){
    check("f864f565f752f771f985f158f736f593f965f949", "f572f754f364f482f721f849f529f637f55", true)
}

#[test]
fn check1() {
    let vs = variants(&"111a".chars().cycle().take(40).collect::<String>());
    let m = vs.values().map(|v| v.len()).max().unwrap();
    println!("{} {}", vs.len(), m)
}

#[test]
fn check2() {
    let v1s = variants("f864f565f752f771f985f158f736f593f965f949");
    let v2s = variants("f572f754f364f482f721f849f529f637f55");
    let s: usize = common(&v1s, &v2s).iter().map(|k| v1s[k].len() * v2s[k].len()).sum();
    println!("{}", s)
}
