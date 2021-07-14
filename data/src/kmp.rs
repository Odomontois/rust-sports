#[derive(Clone, Debug)]
pub struct KMP { pub index: Vec<usize>, pat: Vec<char> }

pub struct KMPIter<A> { pos: usize, kmp: KMP, target: A }

impl KMP {
    pub fn build(pattern: &str) -> KMP {
        if pattern.is_empty() { return KMP { index: vec![], pat: vec![] }; }

        let mut it = KMPIter {
            pos: 0,
            kmp: KMP { index: vec![0], pat: pattern.chars().collect() },
            target: pattern.chars().skip(1),
        };

        while let Some(i) = it.next() {
            it.kmp.index.push(i)
        }

        it.kmp
    }

    pub fn analyze<'a>(self: Self, target: &'a str) -> impl Iterator<Item=usize> + 'a {
        KMPIter { pos: 0, kmp: self, target: target.chars() }
    }
}

impl<A> KMPIter<A> {
    fn valid_pos(&self, c: char) -> bool { c == self.kmp.pat[self.pos] }
}

impl<A: Iterator<Item=char>> Iterator for KMPIter<A> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.target.next()?;
        if self.kmp.pat.len() == 0 { return Some(0); }
        while self.pos != 0 && (self.pos == self.kmp.pat.len() || !self.valid_pos(c)) {
            self.pos = self.kmp.index[self.pos - 1]
        }
        if self.valid_pos(c) { self.pos += 1 }
        Some(self.pos)
    }
}