pub fn detect_capital_use(word: String) -> bool {
    word.chars()
        .fold(Some(State::Start), |s, c| s.and_then(|s| s.feed(c)))
        .is_some()
}

enum State {
    Start,
    Cap,
    Up,
    Low,
}
use State::*;

impl State {
    fn feed(&self, c: char) -> Option<State> {
        match self {
            Start if c.is_uppercase() => Some(Cap),
            Start if c.is_lowercase() => Some(Low),
            Cap | Up if c.is_uppercase() => Some(Up),
            Cap | Low if c.is_lowercase() => Some(Low),
            _ => None,
        }
    }
}


#[test]
fn lolls(){
    println!("si {}", std::mem::size_of::<Option<State>>())
}