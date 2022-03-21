pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
    Graph::new(values, edges).max_quality(max_time)
}

struct Graph {
    values: Vec<i32>,
    adjacent: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    pub fn max_quality(&self, max_time: i32) -> i32 {
        self.iter(max_time, 0, self.values[0], &mut vec![0])
    }
    fn iter(&self, max_time: i32, cur: usize, quality: i32, seen: &mut Vec<usize>) -> i32 {
        let mut best = if cur == 0 { quality } else { 0 };
        let adj = self.adjacent[cur].clone();
        for (i, t) in adj {
            if t > max_time {
                continue;
            }
            let q = if !seen.contains(&i) {
                quality + self.values[i]
            } else {
                quality
            };
            seen.push(i);
            best = self.iter(max_time - t, i, q, seen).max(best);
            seen.pop();
        }
        best
    }
    fn new(values: Vec<i32>, edges: Vec<Vec<i32>>) -> Self {
        let mut adjacent = vec![vec![]; values.len()];
        for e in edges {
            let (x, y, c) = (e[0] as usize, e[1] as usize, e[2]);
            adjacent[x].push((y, c));
            adjacent[y].push((x, c));
        }
        Self { values, adjacent }
    }
}
