#[macro_use]
extern crate scan_fmt;

mod err;
pub mod a;
pub mod b;

use err::T;
fn main() {
    tests(|t| {
        println!("Case #{}: {}", t + 1, b::solution()?);
        Ok(())
    })
    .unwrap();
}

fn tests(mut f: impl FnMut(usize) -> T) -> T {
    let n = scanln_fmt!("{}", usize)?;
    for t in 0..n {
        f(t)?
    }
    Ok(())
}
