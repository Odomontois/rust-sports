pub fn minimum_incompatibility(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let n = nums.len() / k as usize;
    if n == 1 { return 0; }
    min_incompat(nums, n).unwrap_or(-1)
}

fn repetition(nums: &Vec<i32>, x: i32) -> bool {
    nums.iter().rev().fold((x, false), |(p, h), &i| (i, h || i == p)).1
}

fn min_incompat(mut nums: Vec<i32>, n: usize) -> Option<i32> {
    if nums.len() == n { return Some(nums[n - 1] - nums[0]).filter(|_| !repetition(&nums, -1)); }
    let x = nums.pop().unwrap_or(0);
    Combinations::of(nums, n - 1).filter_map(|(c, rest)| {
        if repetition(&c, x) {
            return None;
        }
        let j = x - c[0];
        min_incompat(rest, n).map(|z| j + z)
    }).min()
}

struct Combinations { ix: Vec<usize>, n: usize, k: usize }

impl Combinations {
    fn new(n: usize, k: usize) -> Combinations { Combinations { ix: Vec::new(), n, k } }
    fn of<A>(items: Vec<A>, k: usize) -> CombOf<A> { CombOf { combs: Combinations::new(items.len(), k), items } }

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

    fn pick<A: Clone>(&self, vs: &Vec<A>) -> Vec<A> {
        self.ix.iter().map(|&i| vs[i].clone()).collect()
    }

    fn pick_dual<A: Clone>(&self, vs: &Vec<A>) -> Vec<A> {
        let mut res = Vec::new();
        let mut i = 0;
        for j in 0..self.n {
            if i < self.ix.len() && self.ix[i] == j { i += 1 } else { res.push(vs[j].clone()) }
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

struct CombOf<A> { combs: Combinations, items: Vec<A> }

impl<A: Clone> Iterator for CombOf<A> {
    type Item = (Vec<A>, Vec<A>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.combs.mv() {
            Some((self.combs.pick(&self.items), self.combs.pick_dual(&self.items)))
        } else { None }
    }
}


#[test]
fn min_test() {
    println!("{}", minimum_incompatibility(vec![1, 2, 1, 4], 2));
    println!("{}", minimum_incompatibility(vec![6, 3, 8, 1, 3, 1, 2, 2], 4));
    println!("{}", minimum_incompatibility(vec![5, 3, 3, 6, 3, 3], 3));
}