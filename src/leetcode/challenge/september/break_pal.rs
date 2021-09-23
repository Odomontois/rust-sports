pub fn break_palindrome(palindrome: String) -> String {
    let mut b = palindrome.into_bytes();
    let n = b.len();
    if n <= 1 {
        return "".to_string();
    }
    if let Some(i) = (0..n / 2).chain((n + 1) / 2..n).find(|&i| b[i] != 'a' as u8) {
        b[i] = 'a' as u8
    } else {
        b[n - 1] += 1;
    }
    String::from_utf8(b).unwrap()
}
