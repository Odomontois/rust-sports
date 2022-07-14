use std::ops::Deref;

pub fn longest_repeating<S, I>(s: S, query_characters: S, query_indices: I) -> Vec<i32>
where
    S: Deref<Target = str>,
    I: AsRef<[i32]>,
{
    let mut seg = SegTree::new(s.bytes());
    query_characters
        .bytes()
        .zip(query_indices.as_ref())
        .map(|(byte, &pos)| {
            seg.update(pos as usize, byte);
            seg.longest() as i32
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Default)]
struct Border<A> {
    val: A,
    len: u32,
}
#[derive(Clone, Copy, Debug, Default)]
struct State<A> {
    left: Border<A>,
    right: Border<A>,
    longest: u32,
    len: u32,
}

impl<A: Eq + Copy> State<A> {
    fn new(x: A) -> Self {
        Self {
            left: Border { val: x.clone(), len: 1 },
            right: Border { val: x, len: 1 },
            longest: 1,
            len: 1,
        }
    }
    fn homo(&self) -> bool {
        self.longest == self.len
    }
    fn combine(&self, other: &Self) -> Self {
        let connect = self.right.val == other.left.val;
        let mut longest = self.longest.max(other.longest);
        if connect {
            longest = longest.max(self.right.len + other.left.len);
        }
        let len = self.len + other.len;
        let left = if self.homo() && connect {
            Border {
                val: self.left.val,
                len: self.len + other.left.len,
            }
        } else {
            self.left
        };
        let right = if other.homo() && connect {
            Border {
                val: other.right.val,
                len: other.len + self.right.len,
            }
        } else {
            other.right
        };
        Self {
            left,
            right,
            longest,
            len,
        }
    }
}

#[derive(Debug)]
struct SegTree<A> {
    states: Vec<State<A>>,
    size: usize,
}

impl<A: Eq + Default + Copy> SegTree<A> {
    fn update_iter(&mut self, i: usize, a: A, p: usize, from: usize, to: usize) {
        if to - from == 1 {
            self.states[p] = State::new(a);
            return;
        }
        let m = (from + to) / 2;
        if i < m {
            self.update_iter(i, a, 2 * p + 1, from, m)
        } else {
            self.update_iter(i, a, 2 * p + 2, m, to)
        }
        self.states[p] = self.states[2 * p + 1].combine(&self.states[2 * p + 2]);
    }

    pub fn update(&mut self, i: usize, a: A) {
        self.update_iter(i, a, 0, 0, self.size);
    }

    pub fn longest(&self) -> u32 {
        self.states[0].longest
    }

    pub fn new<I: IntoIterator<Item = A>>(s: I) -> Self
    where
        I::IntoIter: ExactSizeIterator,
    {
        let it = s.into_iter();
        let mut seg = SegTree {
            states: vec![State::default(); it.len() * 4],
            size: it.len(),
        };
        for (i, x) in it.enumerate() {
            seg.update(i, x)
        }

        seg
    }
}

#[test]
fn test1() {
    assert_eq!(vec![3, 3, 4], longest_repeating("babacc", "bcb", [1, 3, 3]))
}

#[test]
fn test2() {
    assert_eq!(vec![2, 3], longest_repeating("abyzz", "aa", [2, 1]))
}
