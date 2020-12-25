trait Boxed {
    fn bx(self) -> Box<Self> where Self: Sized { Box::new(self) }
}

impl<A> Boxed for A {}