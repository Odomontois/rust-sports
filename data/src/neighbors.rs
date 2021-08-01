pub struct Neigbors {
    pub vsize: usize,
    pub hsize: usize,
}

impl Neigbors {
    pub fn size(&self) -> (usize, usize) {
        (self.vsize, self.hsize)
    }

    pub fn matrix<A: AsRef<[B]>, B>(m: &[A]) -> Self {
        let hsize = m.get(0).map(|v| v.as_ref().len()).unwrap_or(0);
        let vsize = m.len();
        Neigbors { vsize, hsize }
    }

    pub fn walk(&self) -> impl Iterator<Item = (usize, usize)> {
        let (n, m) = self.size();
        (0..n).flat_map(move |i| (0..m).map(move |j| (i, j)))
    }

    pub fn neighbors(&self, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
        const NI: [(usize, usize); 4] = [(2, 1), (0, 1), (1, 2), (1, 0)];
        let (n, m) = self.size();
        NI.iter().filter_map(move |&(di, dj)| {
            if i + di > 0 && i + di <= n && j + dj > 0 && j + dj <= m {
                Some((i + di - 1, j + dj - 1))
            } else {
                None
            }
        })
    }
}
