use std::convert::TryFrom;
use std::collections::BTreeMap;

#[allow(dead_code)]
pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut items: Vec<_> = buildings.into_iter()
        .flat_map(|v| <[i32; 3]>::try_from(v.as_slice()))
        .flat_map(|[l, r, h]| vec![(l, h, false), (r, h, true)])
        .collect();

    items.sort();

    let mut events: Vec<_> = items.into_iter().scan(BTreeMap::<i32, i32>::new(), |m, (x, h, end)| {
        if !end {
            if let Some(c) = m.get_mut(&h) { *c += 1 } else { m.insert(h, 1); }
        } else {
            let q = if let Some(c) = m.get_mut(&h) {
                *c -= 1;
                *c
            } else { -1 };
            if q == 0 { m.remove(&h); }
        }
        Some((x, m.keys().cloned().next_back().unwrap_or(0), true))
    }).collect();

    for i in 0..events.len().max(1) - 1 {
        if events[i].0 == events[i + 1].0 {
            events[i].2 = false;
        }
    }
    events = events.into_iter().filter(|t| t.2).collect();
    for i in 0..events.len().max(1) - 1 {
        if events[i].1 == events[i + 1].1 {
            events[i + 1].2 = false;
        }
    }
    events.into_iter().filter_map(|(x, h, yld)| if yld { Some(vec![x, h]) } else { None }).collect()
}

#[test]
fn test_skyline() {
    fn check(xs: &[[i32; 3]]) {
        println!("{:?}", get_skyline(xs.into_iter().map(|x| x.to_vec()).collect()))
    }
    check(&[[2, 9, 10], [3, 7, 15], [5, 12, 12], [15, 20, 10], [19, 24, 8]]);
    check(&[]);
    check(&[[2, 9, 10], [9, 12, 15]]);
    check(&[[1, 5, 3], [1, 5, 3], [1, 5, 3]]);
    check(&[[1, 2, 1], [1, 2, 2], [1, 2, 3]]);
    check(&[[0, 2, 3], [2, 5, 3]]);
}

