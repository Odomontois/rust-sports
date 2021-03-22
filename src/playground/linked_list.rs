use std::iter::{successors, FromIterator};

struct ListNode<A> {
    value: A,
    next: List<A>,
}

struct List<A>(Option<Box<ListNode<A>>>);
impl<A> List<A> {
    fn empty() -> Self {
        List(None)
    }

    fn prepend(self, value: A) -> Self {
        List(Some(Box::new(ListNode { value, next: self })))
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut result = None;
        let mut cur = &mut result;
        let mut iter = iter.into_iter();
        while let Some(value) = iter.next() {
            *cur = Some(Box::new(ListNode {
                value,
                next: List(None),
            }));
            if let Some(b) = cur {
                cur = &mut b.next.0
            }
        }
        List(result)
    }
}

impl<A> Iterator for List<A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        let ListNode { value, next } = *self.0.take()?;
        *self = next;
        Some(value)
    }
}

impl<'a, A: 'a> IntoIterator for &'a List<A> {
    type Item = &'a A;

    type IntoIter = Box<dyn Iterator<Item = &'a A> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(successors(self.0.as_ref(), |x| x.next.0.as_ref()).map(|x| &x.value))
    }
}
