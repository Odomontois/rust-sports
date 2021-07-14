use std::collections::HashMap;

struct Registers<'a>(HashMap<&'a str, i64>);

impl<'a> Registers<'a> {
    fn value(&self, val: Value) -> i64 {
        match val {
            Value::Const(c) => c,
            Value::Rg(nm) => self.0[nm],
        }
    }
    fn to_owned(&self) -> HashMap<String, i64> {
        self.0.iter().map(|(&k, &v)| (k.to_string(), v)).collect()
    }
    fn add(&mut self, name: &'a str, add: i64) {
        *self.0.entry(name).or_insert(0) += add
    }
}

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = Registers(HashMap::new());
    let mut cur = 0;
    let program: Vec<_> = program.into_iter().flat_map(command).collect();
    while cur < program.len() {
        match program[cur] {
            Mov(name, val) => {
                registers.0.insert(name, registers.value(val));
            }
            Inc(name) => registers.add(name, 1),
            Dec(name) => registers.add(name, -1),
            Jnz(check, off) if registers.value(check) != 0 => {
                cur = (cur as i64 + registers.value(off)) as usize;
                continue;
            }
            Jnz(_, _) => {}
        }
        cur += 1
    }

    registers.to_owned()
}
#[derive(Clone, Copy)]
enum Value<'a> {
    Rg(&'a str),
    Const(i64),
}

#[derive(Clone, Copy)]
enum Command<'a> {
    Mov(&'a str, Value<'a>),
    Inc(&'a str),
    Dec(&'a str),
    Jnz(Value<'a>, Value<'a>),
}

fn value(part: &str) -> Value {
    if let Ok(x) = part.parse() {
        Value::Const(x)
    } else {
        Value::Rg(part)
    }
}

use Command::*;
fn command(cmd: &str) -> Option<Command> {
    let mut parts = cmd.split(" ");
    match parts.next()? {
        "mov" => Some(Mov(parts.next()?, value(parts.next()?))),
        "inc" => Some(Inc(parts.next()?)),
        "dec" => Some(Dec(parts.next()?)),
        "jnz" => Some(Jnz(value(parts.next()?), value(parts.next()?))),
        _ => None,
    }
}
