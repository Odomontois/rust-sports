pub fn winner_square_game(n: i32) -> bool {
    let mut win = vec![false; n as usize + 1];
    for i in 0..win.len() {
        if !win[i] {
            (1..)
                .map(|k| i + k * k)
                .take_while(|&sq| i + sq <= n as usize)
                .for_each(|sq| win[i + sq] = true)
        }
    }
    win[n as usize]
}

#[test]
fn count() {
    winner_square_game(1000000);
}
