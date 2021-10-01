pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
    solution(board).unwrap_or(-1)
}

fn solution(board: Vec<Vec<i32>>) -> Option<i32> {
    let n = board.len();
    let x = solve_line(n, |i, j| board[i][j])?;
    let y = solve_line(n, |i, j| board[j][i])?;
    Some(x + y)
}
fn solve_line(n: usize, f: impl Fn(usize, usize) -> i32) -> Option<i32> {
    let match_line = |i: usize| {
        if (0..n).all(|j| f(i, j) == f(0, j)) {
            Some(true)
        } else if (0..n).all(|j| f(i, j) == 1 - f(0, j)) {
            Some(false)
        } else {
            None
        }
    };
    let pattern = (0..n).map(match_line).collect::<Option<Vec<_>>>()?;
    let s = pattern.iter().filter(|x| **x).count();
    let u = (0..n).filter(|&j| j % 2 == 0 && pattern[j]).count();
    let res = if n % 2 == 0 && s * 2 == n {
        u.min(s - u)
    } else if n % 2 == 1 && s * 2 + 1 == n {
        u
    } else if n % 2 == 1 && s * 2 == n + 1 {
        s - u
    } else {
        return None;
    };
    Some(res as i32)
}

#[test]
fn lol() {
    println!(
        "{:?}",
        vec![Some(1), None, Some(2)].into_iter().collect::<Option<Vec<i32>>>()
    )
}
