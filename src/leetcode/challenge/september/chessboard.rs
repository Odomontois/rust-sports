pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
    solution(board).unwrap_or(-1)
}

fn solution(board: Vec<Vec<i32>>) -> Option<i32> {
    let n = board.len();
    let x = solve_line(n, |i, j| board[i][j])?;
    let y = solve_line(n, |j, i| board[j][i])?;
    Some(x + y)
}
fn solve_line(n: usize, f: impl Fn(usize, usize) -> i32) -> Option<i32> {
    let mut pattern = vec![true];
    for i in 1..n {
        if (0..n).all(|j| f(i, j) == f(0, j)) {
            pattern.push(true);
        } else if (0..n).all(|j| f(i, j) == 1 - f(0, j)) {
            pattern.push(false);
        } else {
            return None;
        }
    }
    let s = pattern.iter().filter(|x| **x).count();
    let u = (0..n).filter(|&j| j % 2 == 0 && !pattern[j]).count();
    let res = if n % 2 == 0 && s * 2 == n {
        u.min(n / 2 - u)
    } else if n % 2 == 1 && s * 2 + 1 == n {
        n / 2 - u
    } else if n % 2 == 1 && s * 2 == n + 1 {
        u
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


