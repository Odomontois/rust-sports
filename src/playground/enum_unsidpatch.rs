use enum_dispatch::enum_dispatch;

struct A;
struct B;

#[enum_dispatch]
trait Hello {
    fn hello(&self, other: &impl Hello) -> &str;
}
#[allow(unused_variables)]
impl Hello for A {
    fn hello(&self, other: &impl Hello) -> &str {
        "A"
    }
}

#[allow(unused_variables)]
impl Hello for B {
    fn hello(&self, other: &impl Hello) -> &str {
        "B"
    }
}

#[enum_dispatch(Hello)]
enum AB {
    A(A),
    B(B),
}

#[test]
fn check() {
    let ab : AB = A.into();
    println!("{}", ab.hello(&ab))
}
