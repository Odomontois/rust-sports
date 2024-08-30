use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

trait Foo {
    fn foo(&mut self);
}

impl<'a, A: 'a + Foo + Debug + Clone> CloneableFoo<'a> for A {
    fn clone_box(&self) -> Box<dyn CloneableFoo<'a>> {
        Box::new(self.clone())
    }

    fn clone_arc(&self) -> Arc<dyn CloneableFoo<'a>> {
        Arc::new(self.clone())
    }

    fn clone_arc_mutex(&self) -> Arc<Mutex<dyn CloneableFoo<'a>>> {
        Arc::new(Mutex::new(self.clone()))
    }
}
trait CloneableFoo<'a>: Foo + Debug + 'a {
    fn clone_box(&self) -> Box<dyn CloneableFoo<'a>>;
    fn clone_arc(&self) -> Arc<dyn CloneableFoo<'a>>;
    fn clone_arc_mutex(&self) -> Arc<Mutex<dyn CloneableFoo<'a>>>;
}

impl<'a> Clone for Box<dyn CloneableFoo<'a>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

type DynFoo<'a> = dyn CloneableFoo<'a>;

#[test]
fn check() {
    #[derive(Clone, Debug)]
    struct Bar(i32);
    impl Foo for Bar {
        fn foo(&mut self) {
            self.0 += 1;
        }
    }
    #[derive(Clone, Debug)]
    struct Baz(i32);
    impl Foo for Baz {
        fn foo(&mut self) {
            self.0 -= 1;
        }
    }

    let mut foos: Vec<Box<DynFoo>> = vec![Box::new(Bar(0)), Box::new(Baz(0))];
    foos.iter_mut().for_each(|foo| foo.foo());
    println!("initial {:?}", foos);
    let mut new_foos = foos.clone();
    foos.iter_mut().for_each(|foo| foo.foo());
    foos.iter_mut().for_each(|foo| foo.foo());
    println!("old {foos:?} new {new_foos:?}");
    new_foos.iter_mut().for_each(|foo| foo.foo());    
    println!("pld {foos:?} new {new_foos:?}");
}
