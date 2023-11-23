pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let good = |s: &str, i| str1.chars().take(i).cycle().take(s.len()).eq(s.chars());
    let fine = |&i: &usize| str1.len() % i == 0 && str2.len() % i == 0 && good(&str1, i) && good(&str2, i);
    let l = (1..=str1.len().min(str2.len())).rev().find(fine).unwrap_or(0);
    str1[..l].to_string()
}
