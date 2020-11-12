#[allow(dead_code)]
pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut cur_size = -1;
    let mut pool: Vec<String> = vec![];
    let mut res: Vec<String> = vec![];
    let mut acc = String::with_capacity(max_width as usize);
    for w in words {
        if w.len() as i32 + cur_size + 1 > max_width {
            let fill = max_width - cur_size;
            let spaces = pool.len() as i32 - 1;
            if spaces == 0 {
                acc.push_str(&pool[0]);
                for _ in 0..fill { acc.push(' ') }
            } else {
                let each = fill / spaces + 1;
                let mut add = fill % spaces;
                let m = pool.len() - 1;
                for (j, w1) in pool.into_iter().enumerate() {
                    acc.push_str(&w1);
                    if j < m {
                        for _ in 0..each { acc.push(' ') }
                        if add > 0 { acc.push(' ') }
                        add -= 1;
                    }
                }
            }
            res.push(acc);
            acc = String::with_capacity(max_width as usize);
            pool = vec![];
            cur_size = -1;
        }
        cur_size += w.len() as i32 + 1;
        pool.push(w);
    }
    let m = pool.len() - 1;
    let fill = max_width - cur_size;
    for (j, w) in pool.into_iter().enumerate() {
        acc.push_str(&w);
        if j < m { acc.push(' ') } else { for _ in 0..fill { acc.push(' ') } }
    }
    res.push(acc);
    res
}