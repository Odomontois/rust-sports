pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = edges.len();
    let edges = edges.into_iter().enumerate().map(|(index, v)| Edge {
        from: v[0] as usize,
        to: v[1] as usize,
        weight: v[2],
        index,
    });
    let mut mst = MST::new(n as usize, edges);
    let best = mst.mst([], []);
    let critical = build(m, |i| mst.mst([i], []).iter().all(|&x| Some(x) > best));
    let mut is_critical = vec![false; m as usize];
    critical.iter().for_each(|&i| is_critical[i as usize] = true);
    let pseudo_critical = build(m, |i| mst.mst([], [i]) == best && !is_critical[i]);
    vec![critical, pseudo_critical]
}

fn build(m: usize, mut pred: impl FnMut(usize) -> bool) -> Vec<i32> {
    (0..m).filter_map(|i| pred(i).then(|| i as i32)).collect()
}

#[derive(Clone, Copy, Debug)]
enum DS {
    Root { size: usize },
    Child { parent: usize },
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
    index: usize,
    weight: i32,
}

#[derive(Clone, Debug)]
struct MST {
    ds: Vec<DS>,
    edges: Vec<Edge>,
    index: Vec<usize>,
    n: usize,
}

impl MST {
    fn new(n: usize, edges: impl Iterator<Item = Edge>) -> Self {
        let ds = vec![DS::Root { size: 1 }; n];
        let mut edges: Vec<_> = edges.collect();
        edges.sort_by_key(|e| e.weight);
        let mut index = vec![0; edges.len()];
        for (i, e) in edges.iter().enumerate() {
            index[e.index] = i;
        }
        Self { ds, edges, index, n }
    }

    fn init_ds(&mut self) {
        self.ds.iter_mut().for_each(|x| *x = DS::Root { size: 1 });
    }

    fn root(&mut self, x: usize) -> usize {
        match self.ds[x] {
            DS::Root { .. } => x,
            DS::Child { parent } => {
                let root = self.root(parent);
                if root != parent {
                    self.ds[x] = DS::Child { parent: root }
                }
                root
            }
        }
    }

    fn size(&mut self, p: usize) -> usize {
        match self.ds[p] {
            DS::Root { size } => size,
            DS::Child { .. } => 0,
        }
    }

    fn add_edge(&mut self, e: Edge) -> bool {
        let mut r1 = self.root(e.from);
        let mut r2 = self.root(e.to);
        if r1 == r2 {
            return false;
        }
        let s1 = self.size(r1);
        let s2 = self.size(r2);
        if s1 < s2 {
            std::mem::swap(&mut r1, &mut r2);
        }
        self.ds[r2] = DS::Child { parent: r1 };
        self.ds[r1] = DS::Root { size: s1 + s2 };
        true
    }

    fn add_edge_with(&mut self, e: Edge, remains: &mut usize, sum: &mut i32) {
        if !self.add_edge(e) {
            return;
        }
        *remains -= 1;
        *sum += e.weight;
    }

    fn by_index(&self, i: usize) -> Edge {
        self.edges[self.index[i]]
    }

    fn mst(
        &mut self,
        forbidden: impl IntoIterator<Item = usize> + Copy,
        forced: impl IntoIterator<Item = usize>,
    ) -> Option<i32> {
        self.init_ds();
        let mut remains = self.n - 1;
        let mut sum = 0;
        for i in forced {
            self.add_edge_with(self.by_index(i), &mut remains, &mut sum);
        }
        let edges = std::mem::take(&mut self.edges);
        for &e in &edges {
            if remains == 0 {
                break;
            }
            if forbidden.into_iter().any(|x| x == e.index) {
                continue;
            }
            self.add_edge_with(e, &mut remains, &mut sum);
        }
        self.edges = edges;
        (remains == 0).then(|| sum)
    }
}

#[cfg(test)]
fn check(n: usize, output: [&[i32]; 2], edges: &[[i32; 3]]) {
    assert_eq!(
        output.iter().map(|v| v.to_vec()).collect::<Vec<_>>(),
        find_critical_and_pseudo_critical_edges(n as i32, edges.iter().map(|v| v.to_vec()).collect::<Vec<_>>())
    )
}

#[test]
fn example1() {
    check(
        5,
        [&[0, 1], &[2, 3, 4, 5]],
        &[
            [0, 1, 1],
            [1, 2, 1],
            [2, 3, 2],
            [0, 3, 2],
            [0, 4, 3],
            [3, 4, 3],
            [1, 4, 6],
        ],
    )
}

#[test]
fn example2() {
    check(4, [&[], &[0, 1, 2, 3]], &[[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]])
}

#[test]
fn test1() {
    check(3, [&[0, 1], &[]], &[[0, 1, 1], [1, 2, 1], [0, 2, 100]])
}

#[test]
fn wa1() {
    check(
        6,
        [&[3], &[0, 1, 2, 4, 5, 6]],
        &[
            [0, 1, 1],
            [1, 2, 1],
            [0, 2, 1],
            [2, 3, 4],
            [3, 4, 2],
            [3, 5, 2],
            [4, 5, 2],
        ],
    )
}
