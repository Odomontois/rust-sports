use std::iter::once;
pub fn add_operators(num: String, target: i32) -> Vec<String> {
    (0..1 << (2 * num.len() - 2))
        .filter(|&t: &u32| calc(chars(&num, t)) == Some(target as i64))
        .map(|t| chars(&num, t).collect())
        .collect()
}

fn chars(num: &str, t: u32) -> impl Iterator<Item = char> + '_ {
    const OPS: &[u8] = "+-*".as_bytes();
    let mut digits = num.chars();
    once(digits.next())
        .flatten()
        .chain((0..num.len() - 1).flat_map(move |i| {
            let j = ((t & (3 << (2 * i))) >> 2 * i) as usize;
            OPS.get(j).into_iter().map(|&b| b as char).chain(digits.next())
        }))
}

fn calc(it: impl IntoIterator<Item = char>) -> Option<i64> {
    let mut calc = Calc::new();
    it.into_iter().for_each(|c| calc.feed(c));
    calc.result()
}

#[derive(Default)]
struct Calc {
    add: i64,
    mul: i64,
    num: i64,
    zero: bool,
    err: bool,
}

impl Calc {
    fn new() -> Self {
        Self {
            mul: 1,
            ..Self::default()
        }
    }

    fn push_mul(&mut self) {
        self.mul *= self.num;
        self.num = 0;
        self.zero = false;
    }
    fn push_add(&mut self, init: i64) {
        self.push_mul();
        self.add += self.mul;
        self.mul = init;
    }
    fn feed(&mut self, c: char) {
        match c {
            _ if self.err => {}
            '*' => self.push_mul(),
            '+' => self.push_add(1),
            '-' => self.push_add(-1),
            _ if self.zero => self.err = true,
            '0' if self.num == 0 => self.zero = true,
            c => self.num = self.num * 10 + c.to_digit(10).unwrap_or(0) as i64,
        }
    }
    fn result(mut self) -> Option<i64> {
        self.push_add(1);
        if self.err {
            None
        } else {
            Some(self.add)
        }
    }
}

#[test]
fn test_print() {
    for i in 0..16 {
        println!("{} = {:?}", chars("123", i).collect::<String>(), calc(chars("123", i)))
    }

    for i in 0..16 {
        println!("{} = {:?}", chars("000", i).collect::<String>(), calc(chars("000", i)))
    }
}

#[test]
fn test1() {
    // assert_eq(add_operators("123".to_string(), 6), vec![""]
}
