trait Fhtagn<Args> {
    type Output;
    fn fhtagn(self, args: Args) -> Self::Output;
}

struct Lambada;
impl Fhtagn<(i32,)> for Lambada {
    type Output = i32;
    fn fhtagn(self, args: (i32,)) -> Self::Output {
        args.0
    }
}
// impl Fhtagn<(i64,)> for Lambada {
//     type Output = i64;
//     fn fhtagn(self, args: (i64,)) -> Self::Output {
//         args.0
//     }
// }


fn check<A, R, F: Fhtagn<(A,), Output = R>>(_: F) {}
fn test() {
    check(Lambada)
}
