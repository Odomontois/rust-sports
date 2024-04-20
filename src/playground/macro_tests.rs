trait Num {
    fn one() -> Self;
    fn zero() -> Self;
    fn add(self, rhs: Self) -> Self;
}

macro_rules! num_impl {
    ($($t:ty)*) => {
        $(
            impl Num for $t {
                fn one() -> Self {
                    1
                }
                fn zero() -> Self {
                    0
                }
                fn add(self, rhs: Self) -> Self {
                    self + rhs
                }
            }
        )*
    }
}

num_impl!(i32 i64 i16);

#[cfg(test)]
mod tests {
    use paste::paste;

    use std::fmt::Debug;

    use super::Num;

    fn test1<T: Num + Eq + Debug>() {
        assert_eq!(T::one(), T::one().add(T::zero()));
        assert_eq!(T::one(), T::zero().add(T::one()));
    }

    fn test2<T: Num + Eq + Debug>() {
        assert_eq!(
            T::one().add(T::one()).add(T::one()),
            T::one().add(T::one().add(T::one()))
        );
    }

    macro_rules! tests {
        ($($t:ty)*) => {
        paste! {
                $(
                    #[test]
                    fn [<test1_ $t>]() {
                        test1::<$t>();
                    }
                    #[test]
                    fn [<test2_ $t>]() {
                        test2::<$t>();
                    }
                )*
            }
        }
    }

    tests!(i32 i64 i16);
}
