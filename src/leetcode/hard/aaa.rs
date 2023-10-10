use std::rc::Rc;

trait Foo {
    fn x(&self) -> usize;
}

trait Bar: Foo {
    fn y(&self) -> i32;
}

#[allow(unused_variables)]
fn lol(bar: Rc<dyn Bar>) -> Rc<dyn Foo>{
    todo!()
}
