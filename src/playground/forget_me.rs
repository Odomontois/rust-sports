#[cfg(test)]
use std::mem::forget;

struct Jah(bool);

impl Jah {
    fn new() -> Self {
        Jah(true)
    }
    fn lol<'a>(&'a mut self) -> SmartShit<'a> {
        self.0 = false;
        SmartShit(self)
    }
    fn kek(&mut self) {
        assert!(self.0)
    }
}

struct SmartShit<'a>(&'a mut Jah);

impl Drop for SmartShit<'_> {
    fn drop(&mut self) {
        self.0.0 = true;
    }
}

#[test]
fn main() {
    let mut jah = Jah::new();

    let x = jah.lol();
    forget(x);

    jah.kek()
}
