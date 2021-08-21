pub fn max_product(s: String) -> i64 {
    todo!()
}

struct Manacher<A> {
    elems: Vec<A>,
    res: Vec<usize>,
}

impl<A: Eq> Manacher<A> {
    fn at(&self, i: usize) -> Option<&A> {
        self.elems.get(i).filter(|_| i % 2 == 0)
    }
    fn build(&mut self, s: impl Iterator<Item = A>) {
        let mut cur = 0;
        for (i, c) in s.enumerate() {
            while cur > 0 && (cur * 2 > i || self.at(i - cur * 2) != Some(&c)) {
                
            }
            self.elems.push(c)
        }
    }
}
