use std::borrow::Cow;

fn sub<'a : 'b, 'b, X: Clone>(x: Cow<'a, X>) -> Cow<'b, X> {
    x
}