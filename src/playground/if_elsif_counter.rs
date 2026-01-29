enum Cmd {
    Command(Box<dyn Fn()>),
    If(Box<dyn Fn() -> bool>),
    Elsif(Box<dyn Fn() -> bool>),
    Endif,
}

use Cmd::*;

fn evaluate(commands: impl IntoIterator<Item = Cmd>) {
    let (mut depth, mut skip) = (0, false);

    for cmd in commands {
        match (cmd, depth, skip) {
            (Command(cmd), 0, _) => cmd(),
            (If(cond), 0, _) if !cond() => depth += 1,
            (If(_), _, _) => depth += 1,
            (Elsif(_), 0, _) => {
                skip = true;
                depth = 1;
            }
            (Elsif(cond), 1, false) if cond() => depth = 0,
            (Endif, 0 | 1, _) => {
                skip = false;
                depth = 0
            }
            (Endif, _, _) => depth -= 1,
            _ => {}
        }
    }
}

#[test]
fn kolsky_test() {}
