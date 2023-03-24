pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let mut uf = UnionFind::new(n as usize);
    for road in roads {
        let (x, y, dist) = (road[0] as usize, road[1] as usize, road[2]);
        uf.union(x, y, dist);
    }
    uf.root(0).1.min_dist
}

#[derive(Clone, Copy)]
struct RootInfo {
    size: u32,
    min_dist: i32,
}

impl RootInfo {
    fn add(self, other: Self, dist: i32) -> Self {
        Self {
            size: self.size + other.size,
            min_dist: self.min_dist.min(other.min_dist).min(dist),
        }
    }
}

#[derive(Clone, Copy)]
enum UFData {
    Root(RootInfo),
    Child(usize),
}

struct UnionFind(Vec<UFData>);

impl UnionFind {
    fn new(n: usize) -> Self {
        Self(vec![
            UFData::Root(RootInfo {
                size: 1,
                min_dist: i32::MAX
            });
            n
        ])
    }

    fn root(&mut self, x: usize) -> (usize, RootInfo) {
        match self.0[x] {
            UFData::Root(i) => (x, i),
            UFData::Child(parent) => {
                let (root, i) = self.root(parent);
                self.0[x] = UFData::Child(root);
                (root, i)
            }
        }
    }

    fn union(&mut self, x: usize, y: usize, dist: i32) {
        let (mut x_root, x_i) = self.root(x);
        let (mut y_root, y_i) = self.root(y);
        if x_root == y_root {
            if let UFData::Root(i) = &mut self.0[x_root] {
                i.min_dist = i.min_dist.min(dist);
            }
            return;
        }
        if x_i.size < y_i.size {
            std::mem::swap(&mut x_root, &mut y_root);
        };
        self.0[x_root] = UFData::Root(x_i.add(y_i, dist));
        self.0[y_root] = UFData::Child(x_root);
    }
}
