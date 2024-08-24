use std::ops::Deref;

fn foo<A: Deref<Target = str> + ToString>(a: Box<str>) -> String {
    a.to_string()
}