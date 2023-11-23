#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Snap(i32);

struct SnapVec<A> {
    values: Vec<Vec<(A, Snap)>>,
    snap_id: Snap,
}

impl<A: Copy + Clone + Default> SnapVec<A> {
    fn new(length: i32) -> Self {
        Self {
            values: vec![vec![]; length as usize],
            snap_id: Snap(0),
        }
    }

    fn mut_check(&mut self, index: usize) -> Option<&mut A> {
        let cell = &mut self.values[index];
        let (x, snap) = cell.last_mut()?;
        (*snap == self.snap_id).then(|| x)
    }

    fn set(&mut self, index: i32, val: A) {
        let index = index as usize;
        if let Some(x) = self.mut_check(index) {
            *x = val;
        } else {
            self.values[index].push((val, self.snap_id));
        }
    }

    fn snap(&mut self) -> i32 {
        let Snap(si) = &mut self.snap_id;
        *si += 1;
        return *si - 1;
    }

    fn get(&self, index: i32, snap_id: i32) -> A {
        let cell = &self.values[index as usize];
        let search = cell.binary_search_by_key(&Snap(snap_id), |p| p.1);
        match search {
            Ok(i) => cell[i].0,
            Err(0) => A::default(),
            Err(i) => cell[i - 1].0,
        }
    }
}

type SnapshotArray = SnapVec<i32>;