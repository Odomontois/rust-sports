enum RegExp {
    Normal(char),                 // A character that is not in "()*|."
    Any,                          // Any character
    ZeroOrMore(Box<RegExp>),      // Zero or more occurances of the same regexp
    Or(Box<RegExp>, Box<RegExp>), // A choice between 2 regexps
    Str(Vec<RegExp>),             // A sequence of regexps
}

fn str_to_regex(input: &str) -> Option<RegExp> {
    let mut states = Vec::<State>::new();
    let mut cur: State = <_>::default();
    for c in input.chars() {
        match c {
            '.' => cur.accum.push(RegExp::Any),
            '*' => {
                let last = cur.accum.pop()?;
                if matches!(&last, RegExp::ZeroOrMore(_)) {
                    return None;
                }
                cur.accum.push(RegExp::ZeroOrMore(last.into()));
            }
            '(' => {
                states.push(cur);
                cur = <_>::default();
            }
            ')' => {
                let last = cur.finalize()?;
                cur = states.pop()?;
                cur.accum.push(last);
            }
            '|' if cur.left_or.is_some() => return None,
            '|' => {
                cur.left_or = Some(cur.finalize()?);
                cur.accum.clear();
            }
            _ => cur.accum.push(RegExp::Normal(c)),
        }
    }
    if !states.is_empty() {
        return None;
    }
    cur.finalize()
}

#[derive(Default)]
struct State {
    accum: Vec<RegExp>,
    left_or: Option<RegExp>,
}
use std::mem::take;

impl State {
    fn finalize(&mut self) -> Option<RegExp> {
        let cur = if self.accum.len() <= 1 {
            self.accum.pop()?
        } else {
            RegExp::Str(take(&mut self.accum))
        };
        if let Some(left) = self.left_or.take() {
            RegExp::Or(left.into(), cur.into()).into()
        } else {
            cur.into()
        }
    }
}
