
#[derive(Debug, Clone, Copy)]
enum Foo{
    Bar(i32),
    Baz([i32; 2])
}

#[test]
fn main() {
    let mut foo = Foo::Bar(1);
    match foo {
        Foo::Bar(ref mut x) => *x = 2,
        Foo::Baz(mut x) => {
            x[0] = 1;
        }
    }
    println!("{:?}", foo)
}
