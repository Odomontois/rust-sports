use std::collections::BTreeMap;

pub fn are_occurrences_equal(s: String) -> bool {
    let mut counts = vec![0; 26];
    for c in s.bytes() {
        counts[(c - 'a' as u8) as usize] += 1;
    }
    let c = counts.iter().copied().filter(|&x| x > 0).next().unwrap_or(0);
    counts.into_iter().all(|x| x == 0 || x == c)
}

pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
    let mut place = vec![0; times.len()];

    let mut times: Vec<_> = times
        .into_iter()
        .enumerate()
        .flat_map(|(i, v)| vec![(v[0], i, true), (v[1], i, false)])
        .collect();
    times.sort_by_key(|t| t.0);
    let mut taken = BTreeMap::<i32, i32>::new();
    let mut first = 0;
    for (_, i, enter) in times {
        if i == target_friend as usize {
            return first;
        } else if enter {
            place[i] = first;
            first += 1;
            if let Some(end) = taken.remove(&first) {
                first = end + 1
            }
        } else if place[i] < first {
            if place[i] < first - 1 { 
                taken.insert(place[i] + 1, first - 1);
            }
            first = place[i];
        } else if let Some((&start, &end)) = taken.range(..=place[i]).next_back() {
            taken.remove(&start);
            if start < place[i] {
                taken.insert(start, place[i] - 1);
            }
            if end > place[i] {
                taken.insert(place[i] + 1, end);
            }
        }
    }

    -1
}

#[test]
fn test_smallest_chair() {
    fn check(xs: &[[i32; 2]], tg: i32, exp: i32) {
        assert_eq!(exp, smallest_chair(xs.iter().map(|v| v.to_vec()).collect(), tg))
    }

    check(
        &[
            [18, 19],
            [10, 11],
            [21, 22],
            [5, 6],
            [2, 3],
            [6, 7],
            [43, 44],
            [48, 49],
            [53, 54],
            [12, 13],
            [20, 21],
            [34, 35],
            [17, 18],
            [1, 2],
            [35, 36],
            [16, 17],
            [9, 10],
            [14, 15],
            [25, 26],
            [37, 38],
            [30, 31],
            [50, 51],
            [22, 23],
            [3, 4],
            [27, 28],
            [29, 30],
            [33, 34],
            [39, 40],
            [49, 50],
            [15, 16],
            [4, 5],
            [46, 47],
            [51, 52],
            [32, 33],
            [11, 12],
            [28, 29],
            [47, 48],
            [36, 37],
            [40, 41],
            [42, 43],
            [52, 53],
            [41, 42],
            [31, 32],
            [23, 24],
            [8, 9],
            [19, 20],
            [24, 25],
            [26, 27],
            [45, 46],
            [44, 45],
            [7, 8],
            [13, 14],
            [38, 39],
        ],
        8,
        0,
    );

    check(
        &[
            [82057, 89365],
            [32519, 49655],
            [7573, 20592],
            [8336, 11514],
            [638, 70162],
            [39511, 44262],
            [70399, 79785],
            [8702, 63564],
            [66644, 68330],
            [75156, 90448],
            [11884, 87096],
            [99068, 99875],
            [32555, 54053],
            [5910, 77572],
            [87649, 94390],
            [40084, 56483],
            [7911, 28699],
            [93308, 96154],
            [25562, 39605],
            [73966, 93173],
            [63450, 88007],
            [58811, 80045],
            [56160, 71952],
            [14333, 79867],
            [40342, 76876],
            [69943, 82175],
        ],
        5,
        8,
    );
}
