pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let points = points.into_iter().map(|v| [v[0], v[1]]).collect::<Vec<_>>();
    let dist = |i: usize, j: usize| {
        let ([x1, y1], [x2, y2]) = (points[i], points[j]);
        (x1 - x2).abs() + (y1 - y2).abs()
    };
    let mut edges = (0..points.len())
        .flat_map(|i| (i + 1..points.len()).map(move |j| (-dist(i, j), i, j)))
        .collect::<std::collections::BinaryHeap<_>>();
    let mut uf = UF((0..points.len()).collect());
    let mut dist = 0;
    let mut rem = points.len() - 1;
    while let Some((d, i, j)) = edges.pop() {
        if uf.union(i, j) {
            dist -= d;
            rem -= 1;
        }
        if rem == 0 {
            return dist;
        }
    }

    struct UF(Vec<usize>);

    impl UF {
        fn parent(&mut self, i: usize) -> usize {
            if self.0[i] != i {
                self.0[i] = self.parent(self.0[i]);
            }
            self.0[i]
        }
        fn union(&mut self, i: usize, j: usize) -> bool {
            let (pi, pj) = (self.parent(i), self.parent(j));
            pi != pj && {
                self.0[pi] = pj;
                true
            }
        }
    }

    0
}
