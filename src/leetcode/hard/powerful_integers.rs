pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: impl ToString) -> i64 {
    let s = s.to_string();
    let sp = 10i64.pow(s.len() as u32);
    let s = s.parse().unwrap_or(0i64);
    let base = limit as i64 + 1;
    let q = |x: i64| {
        let r = (x / sp + (x % sp > s) as i64).to_string();
        let step = |(acc, overflow), dig| {
            if overflow {
                (acc * base, true)
            } else if dig >= base {
                ((acc + 1) * base, true)
            } else {
                (acc * base + dig, false)
            }
        };
        r.bytes().map(|b| (b - b'0') as i64).fold((0i64, false), step).0
    };
    q(finish + 1) - q(start)
}

#[test]
fn example1() {
    assert_eq!(number_of_powerful_int(1, 6000, 4, "124"), 5);
}

#[test]
fn example2() {
    assert_eq!(number_of_powerful_int(15, 215, 6, "10"), 2);
}

#[test]
fn example3() {
    assert_eq!(number_of_powerful_int(1000, 2000, 4, "3000  "), 0);
}
