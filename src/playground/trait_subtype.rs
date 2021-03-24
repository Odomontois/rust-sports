use std::ops::Deref;

trait Foo {
    fn get_name(&self) -> &str;
}

impl<B: Deref> Foo for B
where
    B::Target: Foo,
{
    fn get_name(&self) -> &str {
        self.deref().get_name()
    }
}

trait Bar: Foo {
    fn get_age(&self) -> u8;
}

fn accept_foo(foo: Box<dyn Foo>) -> String {
    foo.get_name().to_string()
}

fn accept_bar(bar: Box<dyn Bar>) -> String {
    accept_foo(Box::new(bar))
}
