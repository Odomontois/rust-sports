use itertools::Itertools;
use std::{collections::VecDeque, vec};

fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let bcode = |c: &u8| {
        format!("{:08b}", c)
            .as_bytes()
            .to_vec()
            .into_iter()
            .rev()
            .map(|c| c - '0' as u8)
    };
    let mut input = input.iter().flat_map(bcode);
    let mut brackets = vec![0usize; code.len()];
    let mut stack = vec![];
    let code: Vec<_> = code.chars().collect();
    for (i, &c) in code.iter().enumerate() {
        if c == '[' {
            stack.push(i)
        } else if c == ']' {
            let j = stack.pop().unwrap();
            brackets[i] = j;
            brackets[j] = i;
        }
    }
    let mut output = vec![];
    let mut inst = 0;
    let mut tape = VecDeque::from(vec![false]);
    let mut cur = 0;
    while inst < code.len() {
        match code[inst] {
            '+' => tape[cur] ^= true,
            ',' => tape[cur] = input.next().unwrap_or(0) == 1,
            ';' => output.push(tape[cur] as u8),
            '<' if cur == 0 => tape.push_front(false),
            '<' => cur -= 1,
            '>' => {
                cur += 1;
                if cur == tape.len() {
                    tape.push_back(false)
                }
            }
            '[' if !tape[cur] => inst = brackets[inst],
            ']' if tape[cur] => inst = brackets[inst],
            _ => {}
        }
        inst += 1;
    }
    let res = output.into_iter().chunks(8).into_iter().map(to_num).collect();
    res
}

fn to_num(xs: impl Iterator<Item = u8>) -> u8 {
    xs.zip(0..8).fold(0, |x, (b, i)| x | (b << i))
}

#[test]
fn lol() {
    let mut dd = VecDeque::from(vec![1, 2]);
    println!("{} {}", dd[0], dd[1]);
    dd.push_front(3);
    println!("{} {} {}", dd[0], dd[1], dd[2]);
    dd.push_front(4);
    println!("{} {} {} {}", dd[0], dd[1], dd[2], dd[3]);
    dd.push_back(5);
    println!("{} {} {} {} {}", dd[0], dd[1], dd[2], dd[3], dd[4]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_test_cases() {
        // Hello World Program taken from the official website
        assert_eq!(boolfuck(";;;+;+;;+;+;+;+;+;+;;+;;+;;;+;;+;+;;+;;;+;;+;+;;+;+;;;;+;+;;+;;;+;;+;+;+;;;;;;;+;+;;+;;;+;+;;;+;+;;;;+;+;;+;;+;+;;+;;;+;;;+;;+;+;;+;;;+;+;;+;;+;+;+;;;;+;+;;;+;+;+;", Vec::new()), b"Hello, world!\n", "Your interpreter did not work with the code example provided on the official website");
        // Echo until byte(0) encountered
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>;>;>;>;>;>;>;>;>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]", b"Codewars\x00".to_vec()), b"Codewars");
        // Two numbers multiplier
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>>,>,>,>,>,>,>,>,<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<[>]+<[+<]>>>>>>>>>[+]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<", vec![8, 9]), vec![72]);
    }
}
