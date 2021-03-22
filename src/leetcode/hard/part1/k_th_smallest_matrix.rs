use std::collections::BTreeMap;

pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut m = BTreeMap::new();
    let init = Mat { mat: mat.iter().map(|v| v.as_slice()).collect(), last: 0 };
    m.insert(init.sum(), vec![init]);
    for _ in 0..k - 1 { next_step(&mut m); }

    next_step(&mut m).unwrap_or(-1)
}

#[derive(Clone)]
struct Mat<'a> { mat: Vec<&'a [i32]>, last: usize }

impl<'a> Mat<'a> {
    fn sum(&self) -> i32 { self.mat.iter().map(|v| v[0]).sum() }
    fn switch(&self, i: usize) -> Option<Mat<'a>> {
        let s = self.mat.get(i)?;
        if s.len() == 1 { return None; }
        let mut mat = self.mat.clone();
        mat[i] = &mat[i][1..];
        Some(Mat { mat, last: i })
    }
}

fn next_step(m: &mut BTreeMap<i32, Vec<Mat>>) -> Option<i32> {
    let (&s, v) = m.iter_mut().next()?;
    let next = v.pop()?;
    if v.is_empty() { m.remove(&s); }
    for i in next.last..next.mat.len() {
        if let Some(mat) = next.switch(i) {
            let s1 = mat.sum();
            if let Some(v) = m.get_mut(&s1) {
                v.push(mat);
            } else {
                m.insert(s1, vec![mat]);
            }
        }
    }
    Some(s)
}