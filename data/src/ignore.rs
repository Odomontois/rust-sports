use std::cmp::Ordering;

#[derive(Debug)]
pub struct Ign<A>(pub A);

impl<A> PartialEq for Ign<A> {
    fn eq(&self, _: &Self) -> bool { true }
}

impl<A> Eq for Ign<A> {}

impl<A> PartialOrd for Ign<A> {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> { Some(Ordering::Equal) }
}

impl<A> Ord for Ign<A> {
    fn cmp(&self, _: &Self) -> Ordering { Ordering::Equal }
}