pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    let mut matrix = [[None::<bool>; 3]; 3];
    for (mov, &player) in moves.iter().zip([true, false].iter().cycle()) {
        matrix[mov[0] as usize][mov[1] as usize] = Some(player);
    }

    let all = || {
        line(3, |i, j| [i, j])
            .chain(line(3, |i, j| [j, i]))
            .chain(line(1, |_, j| [j, j]))
            .chain(line(1, |_, j| [j, 2 - j]))
    };
    let check = |p: bool| all().any(|cs| cs.iter().all(|&[i, j]| matrix[i][j] == Some(p)));

    (if check(true) {
        "A"
    } else if check(false) {
        "B"
    } else if moves.len() == 9 {
        "Draw"
    } else {
        "Pending"
    })
    .to_string()
}
fn line(l: usize, f: impl Fn(usize, usize) -> [usize; 2] + 'static) -> impl Iterator<Item = [[usize; 2]; 3]> {
    (0..l).map(move |i| [f(i, 0), f(i, 1), f(i, 2)])
}
