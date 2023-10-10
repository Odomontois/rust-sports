struct X {
    value: [u8; Self::N],
}

struct UninitX(X);

struct InitializedX;

impl X {
    const N: usize = 1 << 24;
    fn init() -> UninitX {
        UninitX(Self { value: [0; Self::N] })
    }
    fn new() -> Result<Self, ()> {
        Ok(Self { value: [0; Self::N] })
        /**/
    }
}

#[test]
#[allow(unused_variables)]
fn main() -> Result<(), ()> {
    let a: Box<X> = Box::new(X::new()?);
    Ok(())
}
