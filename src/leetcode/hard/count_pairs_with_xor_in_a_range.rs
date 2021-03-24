use std::borrow::{Borrow, Cow};

pub fn count_pairs(mut nums: Vec<i32>, low: i32, high: i32) -> i32 {
    nums.sort();
    let depth = nums
        .iter()
        .chain(&[low, high])
        .map(|x| 32 - x.leading_zeros())
        .max()
        .unwrap();

    let mut trie = Trie::new(depth as u8);
    for &num in &nums {
        trie.put(num)
    }
    nums.into_iter()
        .map(|num| trie.count(Some(low), Some(high), num))
        .sum::<usize>() as i32
        / 2
}

#[derive(Clone, Debug, Default)]
struct TrieData {
    count: usize,
    zero: OTData,
    one: OTData,
}

impl TrieData {
    fn put(&mut self, elem: i32, depth: u8) {
        self.count += 1;
        if depth == 0 {
            return;
        }
        let bit = elem & (1 << (depth - 1)) != 0;
        let c = if bit { &mut self.one } else { &mut self.zero };
        if c.is_none() {
            *c = Some(Box::new(TrieData::default()))
        }
        c.as_mut().unwrap().put(elem, depth - 1)
    }

    fn child(&self, bit: bool, depth: u8) -> Trie {
        let data = Cow::Borrowed(if bit { &self.one } else { &self.zero });
        Trie { data, depth }
    }
}

type OTData = Option<Box<TrieData>>;
#[derive(Debug)]
struct Trie<'a> {
    data: Cow<'a, OTData>,
    depth: u8,
}

impl Trie<'static> {
    pub fn new(depth: u8) -> Self {
        Self {
            data: Cow::Owned(None),
            depth,
        }
    }
}

impl Trie<'_> {
    pub fn put(&mut self, elem: i32) {
        if self.data.is_none() {
            self.data = Cow::Owned(Some(Box::new(TrieData::default())));
        }
        self.data.to_mut().as_mut().unwrap().put(elem, self.depth)
    }

    pub fn count(&self, low: Option<i32>, up: Option<i32>, xor: i32) -> usize {
        let b = if let Some(b) = self.data.borrow() { b } else { return 0 };
        if low.is_none() && up.is_none() || self.depth == 0 {
            return b.count;
        }
        let depth = self.depth - 1;
        let mut side = match (low, up) {
            (Some(l), _) if l & (1 << depth) != 0 => Some(true),
            (_, Some(u)) if u & (1 << depth) == 0 => Some(false),
            _ => None,
        };
        let switch = (xor & (1 << depth)) != 0;

        side = side.map(|x| x ^ switch);
        let recur = move |s, low, up| b.child(s, depth).count(low, up, xor);

        if let Some(s) = side {
            recur(s, low, up)
        } else if switch {
            recur(false, None, up) + recur(true, low, None)
        } else {
            recur(false, low, None) + recur(true, None, up)
        }
    }
}

#[test]
fn lol() {
    assert_eq!(count_pairs(vec![1, 4, 2, 7], 2, 6), 6);
    assert_eq!(count_pairs(vec![9, 8, 4, 2, 1], 5, 14), 8);
}
