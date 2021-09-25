use std::collections::HashMap;

use crate::err::*;
type State = HashMap<i32, u32>;

pub fn solution() -> S {
    let (n, _) = scanln_fmt!("{} {}", usize, usize)?;
    let mut free = get_models()?;
    let mut cur: State = free.clone();
    let mut res = 0;
    for _ in 0..n {
        let next = get_models()?;
        res += step(&cur, &mut free, &next);
        cur = next;
    }

    Ok(format!("{}", res))
}

pub fn get_models() -> R<State> {
    let s = scanln_fmt!("{/.*/}", String)?;
    let vs = s.split(" ").map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>()?;
    let mut hm = HashMap::new();
    for x in vs {
        *hm.entry(x).or_insert(0) += 1;
    }
    Ok(hm)
}

fn step(prev: &State, rem: &mut State, next: &State) -> u32 {
    let mut total = 0;
    for (k, &has) in prev {
        let need = next.get(k).copied().unwrap_or(0);
        if has > need {
            let z = rem.entry(*k).or_insert(0);
            let change = (*z).min(has - need);
            *z -= change;
            total += has - need - change;
        }
    }
    total
}
