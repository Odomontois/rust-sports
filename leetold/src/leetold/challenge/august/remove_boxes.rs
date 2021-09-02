pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
    DP::new(boxes).solution()
}

struct DP {
    dp: Vec<Vec<Vec<i32>>>,
    boxes: Vec<i32>,
}
impl DP {
    fn new(boxes: Vec<i32>) -> DP {
        let size = boxes.len();
        DP {
            dp: vec![vec![vec![0; size]; size]; size],
            boxes,
        }
    }
    fn result(&mut self, from: usize, until: usize, remains: usize) -> i32 {
        if from >= until {
            return 0;
        }
        let cached = self.dp[from][until - 1][remains];
        if cached > 0 {
            return cached;
        }
        let mut best = (remains as i32 + 1).pow(2) + self.result(from + 1, until, 0);
        let c = self.boxes[from];
        for split in from + 1..until {
            if self.boxes[split] == c {
                best = best.max(self.result(from + 1, split, 0) + self.result(split, until, remains + 1))
            }
        }
        self.dp[from][until - 1][remains] = best;
        best
    }
    pub fn solution(&mut self) -> i32 {
        self.result(0, self.boxes.len(), 0)
    }
}
