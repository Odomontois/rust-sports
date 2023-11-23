pub fn add_binary(a: String, b: String) -> String {
    let mut bytes: Vec<_> = digits(&a).zip(digits(&b)).scan(0u8, add_step).collect();
    bytes.reverse();
    String::from_utf8(bytes).unwrap_or_default()
}

fn add_step(mem: &mut u8, (ox, oy): (Option<u8>, Option<u8>)) -> Option<u8> {
    (*mem == 1 || ox.is_some() || oy.is_some()).then(|| {
        let s = *mem + ox.unwrap_or(0) + oy.unwrap_or(0);
        *mem = s / 2;
        s % 2 + '0' as u8
    })
}

fn digits(s: &str) -> impl Iterator<Item = Option<u8>> + '_ {
    use std::iter::repeat;
    s.bytes().rev().map(|x| Some(x - '0' as u8)).chain(repeat(None))
}
