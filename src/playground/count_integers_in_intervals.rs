use std::collections::BTreeMap;

#[derive(Debug, Default)]
struct CountIntervals {
    intervals: BTreeMap<i32, i32>,
    total: i32,
}

impl CountIntervals {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, mut left: i32, mut right: i32) {
        let mut removing = Vec::new();
        if let Some((&l, &r)) = self.intervals.range(..=left).rev().next() {
            if r >= left {
                left = l
            }
        }
        for (&l, &r) in self.intervals.range(left..=right) {
            removing.push(l);
            self.total -= r - l + 1;
            right = right.max(r);
        }
        for l in &removing {
            self.intervals.remove(l);
        }
        self.intervals.insert(left, right);
        self.total += right - left + 1;
    }

    fn count(&self) -> i32 {
        self.total
    }
}
