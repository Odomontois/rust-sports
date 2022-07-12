pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, persons: Vec<i32>) -> Vec<i32> {
    let mut points: Vec<_> = flowers.into_iter().flat_map(|v| [(v[0], 1), (v[1] + 1, -1)]).chain([(0, 0)]).collect();
    points.sort_by_key(|(t, _)| *t);
    points.iter_mut().fold(0, |s, (_, cnt)| {
        *cnt += s;
        *cnt
    });
    points.reverse();
    points.dedup_by_key(|(p, _)| *p);
    points.reverse();
    let bloom = |t: i32| {
        let ix = points.binary_search_by_key(&t, |(t, _)| *t).unwrap_or_else(|t| t - 1);
        points.get(ix).map_or(0, |(_, cnt)| *cnt)
    };
    persons.into_iter().map(bloom).collect()
}
