pub fn is_valid_serialization(preorder: String) -> bool {
    let mut it = preorder.split(",");
    Descend(&mut it).go() && it.next().is_none()
}

struct Descend<A>(A);
impl<'a, A> Descend<A>
where
    A: Iterator<Item = &'a str>,
{
    fn go(&mut self) -> bool {
        match self.0.next() {
            Some("#") => true,
            Some(s) => s.parse::<i32>().is_ok() && self.go() && self.go(),
            _ => false,
        }
    }
}
