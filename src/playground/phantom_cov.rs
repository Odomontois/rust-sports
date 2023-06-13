use std::marker::PhantomData;

pub struct Co<T>(PhantomData<T>);
pub struct Contra<T>(PhantomData<fn (T) -> ()>);

pub fn cov_use<'a, 'b: 'a>(a: Co<&'b ()>) -> Co<&'a ()> {
    a
}

pub fn contracov_use<'a: 'b, 'b>(b: Contra<&'b ()>) -> Contra<&'a ()> {
    b
}
