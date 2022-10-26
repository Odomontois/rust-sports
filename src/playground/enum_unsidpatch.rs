use enum_dispatch::enum_dispatch;

struct A;
struct B;

#[enum_dispatch]
trait Hello{
    fn hello(&self, other: &impl Hello) -> &str;
}
impl Hello for A{
    fn hello(&self, other: &impl Hello) -> &str{
        "A"
    }
}

impl Hello for B{
    fn hello(&self, other: &impl Hello) -> &str{
        "B"
    }
}

#[enum_dispatch(Hello)]
enum AB{
    A(A),
    B(B)
}


#[test]
fn check(){
    let ab= AB::A(A);
    println!("{}", ab.hello(&ab))
}