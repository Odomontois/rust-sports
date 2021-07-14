use std::collections::VecDeque;

pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let n = students.len();
    let sq = students.into_iter().filter(|&i| i == 1).count();
    let ci = n - sq;
    if sq == 0 || ci == 0 { return n as i32; };
    (sandwiches.len() - sandwiches.into_iter().scan((sq + 1, ci + 1), |(s, c), x| {
        if x == 1 { *s -= 1 } else { *c -= 1 };
        if *s > 0 || *c > 0 { Some(()) } else { None }
    }, ).count()) as i32
}

pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let (_, s) = customers.iter().fold((0, 0), |(t, w), c| {
        let t1 = (c[0] as i64).max(t) + c[1] as i64;
        (t1, t1 - c[0] as i64 + w)
    });
    s as f64 / customers.len() as f64
}

pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
    let mut firsts = VecDeque::new();
    let mut seconds = VecDeque::new();
    let fc = (k - 1) as usize  / 2;
    let sc = (k - 1) as usize - fc;
    let mut zeros = 0;
    let mut fsum = 0i64;
    let mut fval = 0i64;
    let mut ssum = 0i64;
    let mut sval = 0i64;
    let mut best = std::i64::MAX;
    if k == 1 { return 0; }
    for x in nums.into_iter().skip_while(|&x| x == 0).skip(1) {
        if x == 0 {
            zeros += 1;
            continue;
        }
        fsum += zeros;
        fval += fsum;
        firsts.push_back(zeros);
        zeros = 0;
        if firsts.len() > fc {
            let u = firsts.pop_front().unwrap();
            fval -= u * (fc + 1) as i64;
            fsum -= u;
            seconds.push_back(u);
            ssum += u;
            sval += u * (sc + 1) as i64;
            sval -= ssum;
            if seconds.len() > sc as usize {
                let v = seconds.pop_front().unwrap();
                ssum -= v;
            }
        }
        if seconds.len() == sc {
            best = best.min(fval + sval);
        }
        // dbg!(&x, &fsum, &fval, &ssum, &sval, &zeros, &firsts, &seconds, &best);
    }
    best as i32
}

#[test]
fn test_min_moves() {
    assert_eq!(min_moves(vec![1, 0, 0, 1, 0, 1, 0], 2), 1);
    assert_eq!(min_moves(vec![1, 0, 0, 0, 0, 0, 1, 1], 3), 5);
    assert_eq!(min_moves(vec![1, 1, 0, 1], 2), 0);
}