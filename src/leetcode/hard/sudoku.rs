#[cfg(test)]
use std::cell::RefCell;

pub fn solve_sudoku(_: &mut Vec<Vec<char>>) {}

#[test]
fn lol() {
    println!(
        "{} {}",
        std::mem::size_of::<(usize, usize)>(),
        std::mem::size_of::<RefCell<(usize, usize)>>(),
    );
}
