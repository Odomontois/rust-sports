use std::{cmp::Reverse, collections::BinaryHeap};

pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
    let mut place = vec![-1; times.len()];

    let mut events: BinaryHeap<_> = times
        .iter()
        .enumerate()
        .map(|(i, v)| Reverse((v[0], v[1], true, i)))
        .collect();
    let mut vacant = BinaryHeap::new();
    let mut next = 0;
    while let Some(Reverse((_, tl, enter, i))) = events.pop() {
        if !enter {
            vacant.push(Reverse(place[i]));
            continue;
        }
        if let Some(Reverse(p)) = vacant.pop() {
            place[i] = p
        } else {
            place[i] = next;
            next += 1;
        }
        events.push(Reverse((tl, -1, false, i)));
    }

    place[target_friend as usize]
}

#[cfg(test)]
fn check(xs: &[[i32; 2]], tg: i32, exp: i32) {
    assert_eq!(exp, smallest_chair(xs.iter().map(|v| v.to_vec()).collect(), tg))
}

#[test]
fn test1() {
    check(
        &[
            [4, 5],
            [12, 13],
            [5, 6],
            [1, 2],
            [8, 9],
            [9, 10],
            [6, 7],
            [3, 4],
            [7, 8],
            [13, 14],
            [15, 16],
            [14, 15],
            [10, 11],
            [11, 12],
            [2, 3],
            [16, 17],
        ],
        15,
        0,
    );
}
#[test]
fn test2() {
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
}
#[test]
fn test3() {
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
#[test]
fn test4() {
    check(&[[1, 4], [2, 3], [4, 6]], 1, 1);
}
#[test]
fn test5() {
    check(&[[3, 10], [1, 5], [2, 6]], 0, 2);
}
