pub fn partition_string(s: String) -> i32 {
    s.bytes().fold((0u32, 1), |(s, acc), c| {
            let i = c - b'a';
            if s & (1 << i) != 0 {
                (1 << i, acc + 1)
            } else {
                (s | 1 << i, acc)
            }
        }).1
}
