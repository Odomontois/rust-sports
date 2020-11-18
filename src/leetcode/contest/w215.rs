#[allow(dead_code, unused)]
pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
    0
}

#[allow(dead_code)]
fn next(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item=(usize, usize)> {
    (x + 1..w).map(move |x1| (x1, y)).chain(
        (y + 1..h).flat_map(move |y1| (1..w).map(move |x1| (x1, y1)))
    )
}


#[test]
fn grid_check() {
    print!("jello")
}