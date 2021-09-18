use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::VecDeque;

pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if trees.len() <= 3 {
        return trees;
    }
    let points = trees.into_iter().map(|v| Point(v[0], v[1]));
    if let Some(mut v) = Shell::new(points) {
        v.calc().into_iter().map(|Point(x, y)| vec![x, y]).collect()
    } else {
        vec![]
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i32, i32);
impl Point {
    fn side(&self, a: Point, b: Point) -> Ordering {
        ((b.0 - a.0) * (self.1 - a.1) - (b.1 - a.1) * (self.0 - a.0)).cmp(&0)
    }

    fn dist2(&self, p: Point) -> i64 {
        let mut q = (p.1 - self.1).signum();
        if q == 0 {
            q = (p.0 - self.0).signum()
        }
        q as i64 * (((p.1 - self.1) as i64).pow(2) + ((p.0 - self.0) as i64).pow(2))
    }

    fn on(&self, a: Point, b: Point) -> bool {
        self.side(a, b) == Equal && {
            let d = self.dist2(a);
            let db = b.dist2(a);
            d == 0 || d.signum() == db.signum() && d.abs() <= db.abs()
        }
    }

    fn angle(&self, a: Point) -> f64 {
        ((self.1 - a.1) as f64).atan2((self.0 - a.0) as f64)
    }
}
#[derive(Debug)]
struct Shell {
    pivot: Point,
    points: Vec<Point>,
}

impl Shell {
    fn new(xs: impl Iterator<Item = Point>) -> Option<Self> {
        let mut points: Vec<_> = xs.collect();

        let &pivot = points.iter().max()?;

        points.swap_remove(points.iter().position(|&x| x == pivot)?);

        points.sort_by(|&p1, &p2| {
            pivot.angle(p1).partial_cmp(&pivot.angle(p2)).unwrap().then_with(|| {
                let q = match -pivot.angle(p1).signum() as i64 {
                    0 => 1,
                    q => q,
                };
                (q * p1.dist2(pivot)).cmp(&(q * p2.dist2(pivot)))
            })
        });
        Some(Self { points, pivot })
    }
    fn inside(&self, a: Point, b: Point, p: Point) -> bool {
        p.on(self.pivot, a)
            || p.on(self.pivot, b)
            || p.side(a, b) == self.pivot.side(a, b)
                && p.side(self.pivot, a) == b.side(self.pivot, a)
                && p.side(self.pivot, b) == a.side(self.pivot, b)
    }

    fn calc(&mut self) -> impl IntoIterator<Item = Point> {
        let mut pts = VecDeque::<Point>::new();
        let mut removed: Vec<_> = vec![];
        for &p in &self.points {
            pts.push_front(p);
            while pts.len() > 2 && self.inside(pts[0], pts[2], pts[1]) {
                removed.extend(pts.remove(1));
            }
        }
        let mut remove = true;
        while remove && pts.len() > 2 {
            remove = false;
            if self.inside(pts[pts.len() - 1], pts[1], pts[0]) {
                removed.extend(pts.remove(0));
                remove = true;
            }
            if pts.len() > 2 && self.inside(pts[pts.len() - 2], pts[0], pts[pts.len() - 1]) {
                removed.extend(pts.remove(pts.len() - 1));
                remove = true;
            }
        }
        let add: Vec<_> = removed
            .into_iter()
            .filter(|&p| p.on(self.pivot, pts[0]) || p.on(self.pivot, pts[pts.len() - 1]))
            .collect();
        pts.extend(add);
        pts.push_back(self.pivot);
        pts
    }
}

#[cfg(test)]
fn check(xs: &[[i32; 2]], exp: &[[i32; 2]]) {
    let mut result = outer_trees(xs.iter().map(|v| v.to_vec()).collect());
    result.sort();
    let mut exp: Vec<_> = exp.iter().map(|v| v.to_vec()).collect();
    exp.sort();
    assert_eq!(result, exp);
}

#[test]
fn test1() {
    check(
        &[[1, 1], [2, 2], [2, 0], [2, 4], [3, 3], [4, 2]],
        &[[1, 1], [2, 0], [3, 3], [2, 4], [4, 2]],
    )
}

#[test]
fn test2() {
    check(&[[1, 2], [2, 2], [4, 2]], &[[4, 2], [2, 2], [1, 2]])
}

#[test]
fn test3() {
    check(
        &[[1, 2], [2, 2], [4, 2], [3, 2], [-1, 2], [5, 2]],
        &[[1, 2], [2, 2], [4, 2], [3, 2], [-1, 2], [5, 2]],
    )
}

#[test]
fn test4() {
    check(&[[0, 1], [1, 0], [0, 0], [1, 1]], &[[0, 1], [1, 0], [0, 0], [1, 1]])
}

#[test]
fn test5() {
    check(
        &[
            [0, 10],
            [10, 0],
            [0, 0],
            [10, 10],
            [1, 2],
            [3, 4],
            [5, 5],
            [2, 7],
            [1, 9],
            [3, 1],
        ],
        &[[0, 10], [10, 0], [0, 0], [10, 10]],
    )
}

#[test]
fn test6() {
    check(
        &[[0, 5], [5, 0], [10, 5], [5, 10], [5, 5], [6, 5], [5, 6], [4, 5], [5, 4]],
        &[[0, 5], [5, 0], [10, 5], [5, 10]],
    )
}
