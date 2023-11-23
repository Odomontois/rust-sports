pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    struct BitSet {
        data: Vec<u64>,
        max_set: usize,
    }
    impl BitSet {
        fn new(n: usize) -> Self {
            Self {
                data: vec![0; (n + 63) / 64],
                max_set: 0,
            }
        }

        fn set(&mut self, i: usize) -> bool {
            let (word, bit) = (i / 64, i % 64);
            let cell = &mut self.data[word];
            let res = (*cell & (1 << bit)) != 0;
            *cell |= 1 << bit;
            self.max_set = self.max_set.max(i + 1);
            res
        }

        fn clean(&mut self) {
            for i in 0..(self.max_set + 63) / 64 {
                self.data[i] = 0;
            }
            self.max_set = 0
        }

        fn continious(&mut self) -> bool {
            let (word, bit) = (self.max_set / 64, self.max_set % 64);
            for i in 0..word {
                if self.data[i] != !0 {
                    return false;
                }
            }
            self.data[word] == (1 << bit) - 1
        }
    }

    let bits = &mut BitSet::new(nums.len());
    let mut res = Vec::with_capacity(nums.len());

    let check = |l: usize, r: usize, bits: &mut BitSet| -> bool {
        let nums = || nums[l..=r].iter().copied();
        let first = if let Some(z) = nums().next() { z } else { return true };
        let (min, max) = nums().fold((first, first), |(min, max), x| (min.min(x), max.max(x)));
        if max == min {
            return true;
        }
        let steps = (r - l) as i32;
        let diff = max - min;
        if diff % steps != 0 {
            return false;
        }
        let d = diff / steps;
        for num in nums() {
            let k = num - min;
            if k % d != 0 || bits.set((k / d) as usize) {
                return false;
            }
        }
        bits.continious()
    };
    for (l, r) in l.into_iter().zip(r) {
        res.push(check(l as usize, r as usize, bits));
        bits.clean()
    }
    res
}
