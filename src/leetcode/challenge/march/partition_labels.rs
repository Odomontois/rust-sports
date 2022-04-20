pub fn partition_labels(s: impl AsRef<str>) -> Vec<i32> {
    let mut ints = [None; 26];
    for (i, c) in s.as_ref().bytes().map(|b| (b - 'a' as u8) as usize).enumerate() {
        let i = i as i32;
        let cell = &mut ints[c];
        if let Some((_, end)) = cell {
            *end = i;
        } else {
            *cell = Some((i, i));
        }
    }
    let ints = ints.iter().copied().flatten();
    let mut events: Vec<_> = ints.flat_map(|(s, e)| [(s, false), (e, true)]).collect();
    events.sort();
    events
        .into_iter()
        .scan((-1, 0), |(prev, s), (x, close)| {
            *s += if close { -1 } else { 1 };
            Some((*s == 0).then(|| {
                let res = x - *prev;
                *prev = x;
                res
            }))
        })
        .flatten()
        .collect()
}

#[test]
fn test1() {
    assert_eq!(vec![9, 7, 8], partition_labels("ababcbacadefegdehijhklij"))
}
