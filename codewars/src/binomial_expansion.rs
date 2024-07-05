use std::{error::Error, str::FromStr, sync::RwLock};

use lazy_static::lazy_static;
use num::BigInt;
use regex::Captures;
use std::fmt::Write;
#[derive(Debug)]
struct Parsed {
    a: i64,
    b: i64,
    p: u64,
    c: char,
}

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"\((\-?\d*)?(\w)\+?(\-?\d+)\)\^(\d+)").unwrap();
}

fn expand(expr: &str) -> String {
    let Parsed { a, b, p, c } = parse(expr).unwrap();
    let Some(coefs) = coefs(a, b, p) else {
        return "".to_string();
    };
    coefs
        .into_iter()
        .enumerate()
        .filter_map(|(i, q)| print_coef(q, p as usize - i, c, i == 0))
        .collect()
}

static FACT: RwLock<Vec<BigInt>> = RwLock::new(Vec::new());

fn fact(n: usize) -> Option<BigInt> {
    let read = FACT.read().ok().and_then(|x| x.get(n).cloned());
    read.or_else(|| {
        FACT.write().ok().and_then(|mut fact| {
            if fact.len() == 0 {
                fact.push(BigInt::from(1));
            }
            while fact.len() <= n {
                let next = fact.last()? * fact.len();
                fact.push(next);
            }
            fact.get(n as usize).cloned()
        })
    })
}

fn coef(n: usize, k: usize) -> Option<BigInt> {
    Some(fact(n)? / fact(k)? / fact(n.checked_sub(k)?)?)
}

fn coefs(a: i64, b: i64, p: u64) -> Option<Vec<BigInt>> {
    let a = BigInt::from(a);
    let b = BigInt::from(b);

    (0..=p)
        .map(|i| Some(a.pow((p - i) as u32) * b.pow(i as u32) * coef(p as usize, i as usize)?))
        .collect()
}

fn print_coef(q: BigInt, i: usize, c: char, first: bool) -> Option<String> {
    if q == BigInt::from(0) {
        return None;
    }
    let mut res = String::default();
    if i == 0 || q != BigInt::from(1) {
        if !first {
            write!(res, "{q:+}")
        } else {
            write!(res, "{q}")
        }
        .ok()?;
    }

    if i == 0 {
        Ok(())
    } else if i == 1 {
        write!(res, "{c}")
    } else {
        write!(res, "{c}^{i}")
    }
    .ok()?;

    Some(res)
}

fn get_part<A: FromStr>(c: &Captures, i: usize) -> Result<A, Box<dyn Error>>
where
    A::Err: Error + 'static,
{
    let m = c.get(i).ok_or_else(|| format!("can't get part {i}"))?;
    Ok(m.as_str().parse()?)
}

fn parse(expr: &str) -> Result<Parsed, Box<dyn Error>> {
    let c = RE.captures(expr).ok_or("can't capture")?;
    let a = match get_part::<String>(&c, 1)?.as_str() {
        "-" => -1,
        "" => 1,
        s => s.parse()?,
    };
    Ok(Parsed {
        a,
        c: get_part(&c, 2)?,
        b: get_part(&c, 3)?,
        p: get_part(&c, 4)?,
    })
}

#[test]
fn check1() {
    parse("(-r-12)^3").unwrap();
    parse("(x+0)^0").unwrap();
}

#[test]
fn check2() {
    println!("{:?}", fact(7))
}

#[test]
fn test1() {
    println!("{}", expand("(x+1)^2"));
    println!("{}", expand("(p-1)^3"));
    println!("{}", expand("(2f+4)^6"));
    println!("{}", expand("(-2a-4)^0"));
    println!("{}", expand("(-12t+43)^2"));
    println!("{}", expand("(r+0)^203"));
    println!("{}", expand("(-x-1)^2"));
}
