use std::marker::PhantomData;

trait HK<'a>: 'a {
    type Out<A: 'a>: 'a;
}

trait Coercible<'a, A: 'a>: Sized {
    fn coerce<H: HK<'a>>(_: H::Out<Self>) -> H::Out<A>;

    fn coerce_back<H: HK<'a>>(out: H::Out<A>) -> H::Out<Self> {
        struct HKFn<'a, R, H: HK<'a>>(PhantomData<&'a (R, H)>);
        impl<'a, R, H: HK<'a>> HK<'a> for HKFn<'a, R, H> {
            type Out<A: 'a> = fn(H::Out<A>) -> R;
        }
        (Self::coerce::<HKFn<'a, H::Out<Self>, H>>(|x| x))(out)
    }
}

struct Foo;
// #[derive(Coercible)]
struct Kek(Foo);

impl<'a> Coercible<'a, Foo> for Kek {
    fn coerce<H: HK<'a>>(_: H::Out<Self>) -> H::Out<Foo> {
        unimplemented!()
    }
}

macro_rules! coerce {
    () => {};
}
