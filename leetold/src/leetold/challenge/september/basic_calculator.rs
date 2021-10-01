use std::iter::Peekable;

pub fn calculate(s: String) -> i32 {
    calculator(&mut s.chars().peekable())
}

fn calculator(it: &mut Peekable<impl Iterator<Item = char>>) -> i32 {
    let mut acc = 0;
    let mut sign = 1;
    while let Some(c) = it.next() {
        match c {
            ')' => return acc,
            '(' => {
                acc += sign * calculator(it);
                sign = 1;
            }
            '-' => sign = -1,
            '+' => {}
            c if c.is_whitespace() => {}
            _ => {
                acc += sign * parse_num(c, it);
                sign = 1;
            }
        }
    }
    acc
}

fn parse_num(fst: char, it: &mut Peekable<impl Iterator<Item = char>>) -> i32 {
    let digit = |c| (c as u8 - '0' as u8) as i32;
    let mut acc = digit(fst);
    while let Some(&c) = it.peek() {
        match c {
            c if c.is_digit(10) => acc = acc * 10 + digit(c),
            _ => return acc,
        }
        it.next();
    }
    acc
}

#[cfg(test)]
fn check(exp: i32, s: &str) {
    assert_eq!(exp, calculate(s.to_string()))
}
#[test]
fn test1() {
    check(2, "1 + 1")
}

#[test]
fn test2() {
    check(3, " 2-1 + 2 ")
}

#[test]
fn test3() {
    check(23, "(1+(4+5+2)-3)+(6+8)")
}

#[test]
fn test4(){
    check(-12,"- (3 + (4 + 5))")
}


#[test]
fn test5(){
    check(-4,"- (3 - (4 - 5))")
}
