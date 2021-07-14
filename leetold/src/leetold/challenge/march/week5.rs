use std::iter::repeat;

pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
    let rep: Vec<_> = repeat('*' as u8).take(stamp.len()).collect();
    let mut target: Vec<_> = target.into_bytes();
    let stamp = stamp.as_bytes();
    let mut result = vec![];
    while target.iter().any(|&c| c != '*' as u8) {
        let next = replace(stamp, &mut target, &rep);
        if next.is_empty() {
            return vec![];
        }
        result.extend(next);
    }
    result.reverse();
    result
}

pub fn replace(stamp: &[u8], target: &mut [u8], rep: &[u8]) -> Vec<i32> {
    (0..target.len() - stamp.len() + 1)
        .filter_map(|i| {
            if matches(stamp, &target[i..]) {
                target[i..i + stamp.len()].copy_from_slice(rep);
                Some(i as i32)
            } else {
                None
            }
        })
        .collect()
}

pub fn matches(stamp: &[u8], target: &[u8]) -> bool {
    let mut found = false;
    for i in 0..stamp.len() {
        if target[i] == stamp[i] {
            found = true
        } else if target[i] != '*' as u8 {
            return false;
        }
    }
    found
}

#[test]
fn test_stamp() {
    fn check(stamp: &str, target: &str, exp: &[i32]) {
        assert_eq!(moves_to_stamp(stamp.to_string(), target.to_string()), exp.to_vec())
    }
    check("aq", "aqaaqaqqqaqqaaq", &[8, 11, 7, 1, 12, 10, 6, 4, 2, 13, 9, 5, 3, 0]);
    check("abc", "ababc", &[0, 2]);
    check("abca", "aabcaca", &[0, 3, 1]);
}
