pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let mut sum_tree = SumTree::new(&nums);
    let mut res: Vec<_> = nums.iter().rev().map(|&x| sum_tree.insert(x)).collect();
    res.reverse();
    res
}

struct SumTree {
    elems: Vec<i32>,
    from: i32,
    to: i32,
}

impl SumTree {
    fn new(elems: &[i32]) -> Self {
        let from = elems.iter().min().copied().unwrap_or(0);
        let to = elems.iter().max().copied().unwrap_or(0) + 1;
        let elems = vec![0; (to - from + 1) as usize * 4];
        Self { elems, from, to }
    }
    fn insert(&mut self, x: i32) -> i32 {
        self.ins_iter(x, self.from, self.to, 0)
    }
    fn ins_iter(&mut self, x: i32, from: i32, to: i32, pos: usize) -> i32 {
        self.elems[pos] += 1;
        if to - from == 1 {
            return 0;
        }
        let m = (from + to) / 2;
        if x >= m {
            self.elems[pos * 2 + 1] + self.ins_iter(x, m, to, pos * 2 + 2)
        } else {
            self.ins_iter(x, from, m, pos * 2 + 1)
        }
    }
}
