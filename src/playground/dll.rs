use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Default, Clone)]
struct DLL<A> {
    value: A,
    next: Option<Rc<DLLCell<A>>>,
    prev: Option<Weak<DLLCell<A>>>,
}

type DLLCell<A> = RefCell<DLL<A>>;
struct DLLPointer<A>(Rc<DLLCell<A>>);

impl<A> DLLPointer<A> {
    fn try_insert_after(&mut self, value: A) -> Option<Self> {
        let mut this = self.0.borrow_mut();

        let node: Rc<DLLCell<A>> = Rc::new(
            DLL {
                value,
                next: this.next.take(),
                prev: Some(Rc::downgrade(&self.0)),
            }
            .into()
        );
        this.next = Some(node.clone());
        Some(Self(node))
    }
}
