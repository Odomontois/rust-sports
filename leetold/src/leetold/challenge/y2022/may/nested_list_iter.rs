use self::NestedInteger::*;
use std::iter::{once, Peekable};

pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

type It = Box<dyn Iterator<Item = i32> + 'static>;
struct NestedIterator(Peekable<It>);

fn iter(ni: Vec<NestedInteger>) -> It {
    Box::new(ni.into_iter().flat_map(|x| match x {
        Int(x) => Box::new(once(x)),
        List(v) => iter(v),
    }))
}

impl NestedIterator {
    fn new(ni: Vec<NestedInteger>) -> Self {
        Self(iter(ni).peekable())
    }

    fn next(&mut self) -> i32 {
        self.0.next().unwrap_or(0)
    }

    fn has_next(&mut self) -> bool {
        self.0.peek().is_some()
    }
}
