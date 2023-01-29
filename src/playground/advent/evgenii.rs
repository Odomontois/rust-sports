use itertools::Itertools;


type Item = char;
type Stack = Vec<char>;

fn parse_state(lines: Vec<String>) -> Vec<Stack> {
    let mut stacks = vec![Stack::default(); lines[0].len() / 4];
    for line in lines {
        for i in 0..line.len() / 4 {
            let s = line[i * 4..i * 4 + 3].chars().collect_vec();
            match s[..] {
                ['[', c, ']'] => stacks[i].push(c),
                _ => {}
            }
        }
    }
    stacks
}
