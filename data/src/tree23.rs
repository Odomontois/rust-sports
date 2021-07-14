use std::ops::Add;

#[allow(dead_code)]
enum Tree23<A: Measured> {
    Leaf(A),
    Node(BT23<A>, A, BT23<A>, Option<(A, BT23<A>)>, <A as Measured>::Measure),
}

type BT23<A> = Box<Tree23<A>>;

trait Combine {
    #[allow(dead_code)]
    fn combine(&self, other: &Self) -> Self;
}

trait Measured: Ord where Self::Measure: Combine {
    type Measure;
    fn measure(&self) -> Self::Measure;
}

use Tree23::*;
use std::mem::swap;

impl<A: Measured> Tree23<A> {
    #[allow(dead_code)]
    pub fn insert(&mut self, a: A) {}

    #[allow(dead_code)]
    fn insert_iter(&mut self, mut a: A) -> Option<Self> {
        match self {
            Leaf(x) => {
                if *x > a {
                    let y = &mut a;
                    swap(x, y)
                }
                Some(Leaf(a))
            }
            Node(tl, x, tm, or, m) if *x < a => {
                let mut u = Box::new(tl.insert_iter(a)?);
                if let Some((y, tr)) = or{

                } else {
                    let ur = &mut u;
                    swap(tm, ur);
                };
                None
            }
            _ => None
        }
    }
}
