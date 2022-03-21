use std::ops::Sub;

pub fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32 {
    AndSum::calc(nums, num_slots as usize)
}

struct AndSum {
    cache: Vec<Vec<Option<usize>>>,
    nums: Vec<i32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct IntSet(usize);

impl IntSet {
    fn elem(self) -> impl Fn(&usize) -> bool + 'static {
        move |&i| self.0 & (1 << i) != 0
    }

    fn ones(self) -> usize {
        self.0.count_ones() as usize
    }
}

impl Sub<usize> for IntSet {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self {
        Self(self.0 & !(1 << rhs))
    }
}

impl AndSum {
    fn calc(nums: Vec<i32>, slots: usize) -> i32 {
        let pn = 1 << nums.len();
        let cache = vec![vec![None; pn]; slots];
        Self { nums, cache }.calc_rec(IntSet(pn - 1), slots) as i32
    }

    fn ac(&self, i: usize, slots: usize) -> usize {
        self.nums[i] as usize & slots
    }

    fn calc_rec(&mut self, set: IntSet, slots: usize) -> usize {
        if slots == 0 || set == IntSet(0) {
            return 0;
        }
        if let Some(cached) = self.cache[slots - 1][set.0] {
            return cached;
        }

        let res = (0..self.nums.len())
            .filter(set.elem())
            .flat_map(|i| {
                let skip = (set.ones() < slots * 2 - 1).then(|| self.calc_rec(set, slots - 1));
                let ix = self.ac(i, slots);
                let take_one = (set.ones() < slots * 2).then(|| self.calc_rec(set - i, slots - 1) + ix);
                let js = (0..i).filter(set.elem());
                let take_two = js.map(|j| self.calc_rec(set - i - j, slots - 1) + ix + self.ac(j, slots));
                skip.into_iter().chain(take_one).chain(take_two.max())
            })
            .max()
            .unwrap_or(0);

        // let nums: Vec<_> = (0..self.nums.len()).filter(set.elem()).map(|i| self.nums[i]).collect();
        // println!("{nums:?} {slots} = {res}");

        self.cache[slots - 1][set.0] = Some(res);

        res
    }
}

#[test]
fn check() {
    assert_eq!(9, maximum_and_sum(vec![1, 2, 3, 4, 5, 6], 3))
}
