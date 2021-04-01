use std::future::Future;
use std::sync::{Arc, Mutex};

fn main() {
    let foo = Arc::new(Mutex::new(0_u64));

    let _ = repl(Box::new(move || {
        let foo = foo.clone();
        async move {
            let _ = foo.clone();
        }
    }));
}

pub fn repl<H, Fut>(_handler: H)
where
    H: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
}
