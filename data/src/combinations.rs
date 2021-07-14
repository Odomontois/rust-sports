pub struct Combinations { ix: Vec<usize>, n: usize, k: usize }

impl Combinations {
    pub fn new(n: usize, k: usize) -> Combinations { Combinations { ix: Vec::new(), n, k } }

    pub fn dual_slice<A>(xs: &[A], k: usize) -> impl Iterator<Item=(Vec<&A>, Vec<&A>)> { Self::new(xs.len(), k).with_dual(move |i| &xs[i]) }
    pub fn dual_vec_copy<A: Copy>(xs: Vec<A>, k: usize) -> impl Iterator<Item=(Vec<A>, Vec<A>)> { Self::new(xs.len(), k).with_dual(move |i| xs[i]) }
    pub fn dual_slice_copy<'a, A: Copy>(xs: &'a [A], k: usize) -> impl Iterator<Item=(Vec<A>, Vec<A>)> + 'a { Self::new(xs.len(), k).with_dual(move |i| xs[i]) }

    pub fn slice<A>(xs: &[A], k: usize) -> impl Iterator<Item=Vec<&A>> { Self::new(xs.len(), k).of(move |i| &xs[i]) }
    pub fn vec_copy<A: Copy>(xs: Vec<A>, k: usize) -> impl Iterator<Item=Vec<A>> { Self::new(xs.len(), k).of(move |i| xs[i]) }
    pub fn slice_copy<'a, A: Copy>(xs: &'a [A], k: usize) -> impl Iterator<Item=Vec<A>> + 'a { Self::new(xs.len(), k).of(move |i| xs[i]) }

    pub fn of<A, F>(self, picker: F) -> CombOf<F> where F: Fn(usize) -> A { CombOf { combs: self, picker } }
    pub fn with_dual<A, F>(self, picker: F) -> CombWithDual<F> where F: Fn(usize) -> A { CombWithDual { combs: self, picker } }

    fn dual(n: usize, v: &Vec<usize>) -> Vec<usize> {
        let mut res = Vec::new();
        let mut i = 0;
        for j in 0..n {
            if v[i] == j { i += 1 } else { res.push(j) }
        }

        res
    }

    fn mv(&mut self) -> bool {
        if self.ix.is_empty() {
            self.ix = (0..self.k).collect();
            return true;
        }
        let mut m = self.n;
        let l = self.ix.len();
        let mut k = l;
        while k > 0 && self.ix[k - 1] + 1 == m {
            k -= 1;
            m -= 1;
            self.ix.pop();
        }
        if k == 0 { return false; }
        self.ix[k - 1] += 1;
        let s = self.ix[k - 1] + 1;

        for u in s..s + l - k { self.ix.push(u) }
        true
    }

    fn pick<A, F>(&self, picker: F) -> Vec<A> where F: Fn(usize) -> A {
        self.ix.iter().cloned().map(picker).collect()
    }

    fn pick_dual<A, F>(&self, picker: F) -> Vec<A> where F: Fn(usize) -> A {
        let mut res = Vec::new();
        let mut i = 0;
        for j in 0..self.n {
            if i < self.ix.len() && self.ix[i] == j { i += 1 } else { res.push(picker(j)) }
        }

        res
    }
}

impl Iterator for Combinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mv() { Some(self.ix.clone()) } else { None }
    }
}

pub struct CombOf<F> { combs: Combinations, picker: F }

impl<A, F> Iterator for CombOf<F> where F: Fn(usize) -> A {
    type Item = Vec<A>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.combs.mv() {
            Some(self.combs.pick(&self.picker))
        } else { None }
    }
}


pub struct CombWithDual<F> { combs: Combinations, picker: F }

impl<A, F> Iterator for CombWithDual<F> where F: Fn(usize) -> A {
    type Item = (Vec<A>, Vec<A>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.combs.mv() {
            Some((self.combs.pick(&self.picker), self.combs.pick_dual(&self.picker)))
        } else { None }
    }
}

#[test]
fn test() {
    for comb in Combinations::new(6, 6) { println!("{:?}", comb) }
    for comb in Combinations::new(6, 3) { println!("{:?}", comb) }
    for comb in Combinations::dual_vec_copy((1..7).collect(), 3) { println!("{:?}", comb) }
    println!("{}", Combinations::new(6, 3).count())
}