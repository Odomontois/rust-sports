pub fn count_of_substrings(word: String, k: i32) -> i64 {
    let word = word.as_bytes();
    fn group(c: u8) -> usize {
        match c {
            b'a' => 5,
            b'e' => 1,
            b'i' => 2,
            b'o' => 3,
            b'u' => 4,
            _ => 0,
        }
    }
    let mut groups = [0; 6];
    let [mut start, mut mid] = [0; 2];
    let mut total = 0i64;
    for &b in word {
        groups[group(b)] += 1;
        if groups[0] < k {
            continue;
        }
        if groups[0] > k {
            while groups[0] > k {
                groups[group(word[mid])] -= 1;
                mid += 1;
            }
            start = mid;
        }
        if groups[0] == k && groups[1..].iter().all(|&g| g > 0) {
            while let Some(g) = Some(group(word[mid])).filter(|&g| g != 0 && groups[g] > 1) {
                groups[g] -= 1;
                mid += 1;
            }
            total += (mid - start + 1) as i64;
        }
    }
    total
}

#[cfg(test)]
macro_rules! check {
    ($s: expr, $k: expr, $t: expr) => {
        assert_eq!($t, count_of_substrings($s.to_string(), $k))
    };
}

#[test]
fn example1() {
    check!("abcde", 1, 0)
}

#[test]
fn example2() {
    check!("aeiou", 0, 1)
}

#[test]
fn example3() {
    check!("ieaouqqieaouqq", 1, 3)
}

#[test]
fn test1() {
    check!("ssaeiuoaeiuoasaeoiuaoueioss", 2, 37)
}

#[test]
fn wa1() {
    check!("iqeaouqi", 2, 3)
}
