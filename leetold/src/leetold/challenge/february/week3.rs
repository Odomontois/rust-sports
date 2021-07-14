pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
    a.windows(3)
        .fold((1, 0), |(st, acc), w| {
            if w[2] - w[1] == w[1] - w[0] {
                (st + 1, acc + st)
            } else {
                (1, acc)
            }
        })
        .1
}

pub fn min_remove_to_make_valid(s: String) -> String {
    let mut res: Vec<_> = s.chars().map(Some).collect();
    let mut open = vec![];
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => open.push(i),
            ')' if open.pop().is_none() => res[i] = None,
            _ => (),
        }
    }
    for i in open {
        res[i] = None
    }
    res.into_iter().flatten().collect()
}

pub fn roman_to_int(s: String) -> i32 {
    let mut prev = 0;
    let mut acc = 0;
    for c in s.chars().rev() {
        let cur = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        acc += if cur >= prev { cur } else { -cur };
        prev = cur;
    }
    acc
}

#[test]
fn check_roman() {
    fn check(s: &str, exp: i32) {
        assert_eq!(roman_to_int(s.to_string()), exp)
    }
    check("III", 3);
    check("IV", 4);
    check("IX", 9);
    check("LVIII", 58);
    check("MCMXCIV", 1994);
}
