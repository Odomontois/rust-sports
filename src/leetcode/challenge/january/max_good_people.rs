pub fn maximum_good(statements: Vec<Vec<i32>>) -> i32 {
    (0i32..1 << statements.len())
        .filter(|&v| {
            statements
                .iter()
                .enumerate()
                .flat_map(|(i, vs)| vs.iter().enumerate().map(move |(j, &s)| (i, j, s)))
                .all(|(i, j, s)| ok(s, v & (1 << i) != 0, v & (1 << j) != 0))
        })
        .map(|v| v.count_ones() as i32)
        .max()
        .unwrap_or(0)
}

fn ok(statement: i32, teller: bool, subject: bool) -> bool {
    match statement {
        0 => teller == false || subject == false,
        1 => teller == false || subject == true,
        _ => true,
    }
}
