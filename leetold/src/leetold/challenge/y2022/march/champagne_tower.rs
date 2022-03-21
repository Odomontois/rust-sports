pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let mut cur = vec![poured as f64];
    for _ in 0..query_row {
        let mut next = vec![0.0; cur.len() + 1];
        for (i, x) in cur.into_iter().enumerate() {
            let overflow = (x - 1.0).max(0.0) / 2.0;
            next[i] += overflow;
            next[i + 1] += overflow;
        }
        cur = next
    }
    cur[query_glass as usize].min(1.0)
}
