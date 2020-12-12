use std::fmt::Display;

pub fn find_min_step(board: String, hand: String) -> i32 {
    let mut hv: Vec<_> = hand.chars().collect();
    hv.sort();
    min_step((0, board.chars().collect()), hv).unwrap_or(-1)
}


fn min_step((start, board): (usize, Vec<char>), hand: Vec<char>) -> Option<i32> {
    // println!("{} {} {}", vs(&board), vs(&hand), start);
    if board.is_empty() { return Some(0); }
    if hand.is_empty() { return None; }

    (start..board.len()).filter_map(|b|
        (0..hand.len())
            .filter_map(|h|
                if board[b] != hand[h] && (b == 0 || board[b - 1] != hand[h]) {
                    let mut next = board.clone();
                    next.insert(b, hand[h]);
                    min_step((b, next), remove(&hand, h)).map(|x| x + 1)
                } else if board[b] != hand[h] || b + 1 < board.len() && board[b + 1] == hand[h] || h > 0 && hand[h - 1] == hand[h] {
                    None
                } else if b > 0 && board[b - 1] == hand[h] {
                    min_step(clean(&board, b), remove(&hand, h)).map(|x| x + 1)
                } else if h + 1 < hand.len() && board[b] == hand[h + 1] {
                    min_step(clean(&board, b), remove(&remove(&hand, h), h)).map(|x| x + 2)
                } else { None }
            ).min()
    ).min()
}

fn remove(h: &Vec<char>, i: usize) -> Vec<char> {
    let mut res = h.clone();
    res.remove(i);
    res
}

fn clean(board: &Vec<char>, i: usize) -> (usize, Vec<char>) {
    let mut s = i;
    let mut e = i;
    while s > 0 && board[s - 1] == board[i] { s -= 1 }
    while s > 0 && e + 1 < board.len() && board[s - 1] == board[e + 1] &&
        (s > 1 && board[s - 2] == board[e + 1] || e + 2 < board.len() && board[s - 1] == board[e + 2]) {
        s -= 1;
        e += 1;
        while s > 0 && board[s - 1] == board[s] { s -= 1 }
        while e + 1 < board.len() && board[e] == board[e + 1] { e += 1 }
    }
    let mut res = board.clone();
    res.drain(s..=e);
    // println!("clean {} {} {:?} {}", vs(board), i, vs(&res), s);
    (s, res)
}

fn vs<A: Display>(cs: &Vec<A>) -> String {
    cs.iter().map(|c| format!("{}", c)).collect()
}


#[test]
fn zuma_test() {
    fn check(board: &str, hand: &str) {
        println!("{} {} {}", board, hand, find_min_step(board.to_string(), hand.to_string()))
    }
    check("WRRBBW", "RB");
    check("R", "RR");
    check("WWRRBBWW", "WRBRW");
    check("RRWWRRBBRR", "WB");
}