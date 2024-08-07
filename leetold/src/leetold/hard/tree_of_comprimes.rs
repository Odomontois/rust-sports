use std::{usize, vec};

pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let tab: Vec<Vec<usize>> = (0..=50)
        .map(|i| (1..=50).filter(|&j| coprime(i, j as i32)).collect())
        .collect();
    let mut adj = vec![vec![]; nums.len()];
    for v in edges {
        let (x, y) = (v[0] as usize, v[1] as usize);
        adj[x].push(y);
        adj[y].push(x);
    }
    let mut seen = vec![false; nums.len()];
    let mut anc = vec![vec![]; 51];
    let mut stack = vec![Ok::<_, usize>((0, 0))];
    let mut res = vec![-1; nums.len()];
    while let Some(n) = stack.pop() {
        let (i, l) = match n {
            Err(i) => {
                anc[nums[i] as usize].pop();
                continue;
            }
            Ok(p) => p,
        };

        seen[i] = true;
        let ni = nums[i] as usize;
        res[i] = tab[ni]
            .iter()
            .flat_map(|&j| anc[j].last())
            .max_by_key(|(_, l)| l)
            .map(|&(i, _)| i as i32)
            .unwrap_or(-1);

        anc[ni].push((i, l));

        stack.push(Err(i));

        for &j in &adj[i] {
            if seen[j] {
                continue;
            }
            stack.push(Ok((j, l + 1)));
        }
    }

    res
}

fn coprime(x: i32, y: i32) -> bool {
    y == 0 && x == 1 || y != 0 && coprime(y, x % y)
}

#[test]
fn test_coprimes() {
    fn check(nums: &[i32], edges: &[[i32; 2]], exp: &[i32]) {
        assert_eq!(
            &get_coprimes(nums.to_vec(), edges.iter().map(|v| v.to_vec()).collect()),
            exp
        )
    }
    // check(&[2, 3, 3, 2], &[[0, 1], [1, 2], [1, 3]], &[-1, 0, 0, 1]);
    // check(
    //     &[5, 6, 10, 2, 3, 6, 15],
    //     &[[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
    //     &[-1, 0, -1, 0, 0, 0, -1],
    // );

    check(
        &[
            36, 49, 26, 50, 23, 41, 1, 33, 7, 8, 26, 14, 43, 49, 21, 36, 2, 11, 33, 8, 34, 20, 11, 12, 39, 46, 4, 47,
            31, 33, 38, 39, 13, 14, 1, 5, 4, 44, 3, 13, 25, 34, 2, 40, 35, 4, 13, 37, 12, 26, 27, 5, 7, 1, 42, 44, 41,
            43, 43, 8, 50, 8, 44, 40, 11, 1, 17, 34, 25, 8, 14, 9, 19, 6, 44, 38, 49, 50, 27, 50, 25, 10, 1, 41, 30, 5,
            26, 38, 6, 48, 40, 13, 11, 44, 44, 14, 48, 16, 3, 24, 4, 26, 36, 7, 35, 50, 34, 32,
        ],
        &[
            [99, 0],
            [64, 99],
            [91, 64],
            [55, 91],
            [94, 0],
            [37, 94],
            [58, 37],
            [10, 58],
            [6, 10],
            [56, 10],
            [45, 56],
            [36, 45],
            [40, 45],
            [49, 56],
            [101, 49],
            [39, 37],
            [69, 39],
            [30, 69],
            [34, 30],
            [71, 39],
            [98, 39],
            [31, 98],
            [22, 31],
            [102, 22],
            [3, 98],
            [107, 3],
            [50, 107],
            [18, 50],
            [16, 18],
            [85, 107],
            [15, 85],
            [67, 107],
            [42, 67],
            [72, 67],
            [12, 67],
            [79, 12],
            [53, 79],
            [41, 53],
            [68, 53],
            [59, 53],
            [78, 59],
            [5, 59],
            [105, 5],
            [103, 105],
            [24, 79],
            [70, 24],
            [4, 70],
            [33, 70],
            [54, 70],
            [13, 70],
            [46, 13],
            [74, 46],
            [93, 74],
            [90, 93],
            [1, 90],
            [32, 1],
            [52, 1],
            [38, 90],
            [57, 38],
            [95, 90],
            [14, 95],
            [75, 14],
            [66, 75],
            [97, 95],
            [2, 97],
            [86, 97],
            [25, 97],
            [100, 25],
            [61, 100],
            [8, 61],
            [104, 93],
            [80, 104],
            [82, 104],
            [19, 74],
            [81, 19],
            [62, 81],
            [87, 81],
            [48, 87],
            [77, 48],
            [51, 77],
            [7, 81],
            [63, 7],
            [27, 63],
            [26, 63],
            [28, 26],
            [84, 7],
            [11, 84],
            [20, 7],
            [89, 20],
            [92, 20],
            [76, 92],
            [44, 92],
            [83, 44],
            [65, 19],
            [47, 65],
            [60, 47],
            [23, 60],
            [43, 23],
            [21, 43],
            [17, 21],
            [35, 23],
            [9, 35],
            [29, 9],
            [73, 29],
            [88, 60],
            [96, 65],
            [106, 19],
        ],
        &[
            -1, 90, 13, 98, 70, 59, 10, 81, 61, 35, 58, 7, 67, 24, 90, 85, 18, 21, 107, 46, 7, 47, 31, 47, 79, 46, 7,
            63, 26, 9, 39, 37, 1, 24, 30, 23, 56, -1, 90, 37, 45, 53, 98, 47, 92, 56, 13, 65, 46, 56, 107, 48, 90, 79,
            12, 91, 10, 38, 37, 53, 47, 46, 46, 7, 99, 19, 75, 98, 53, 39, 24, 39, 67, 35, 46, 14, 92, 46, 59, 12, 93,
            46, 104, 44, 46, 107, 13, 46, 47, 46, 46, 64, 20, 46, -1, 46, 65, 46, 39, -1, 46, 56, 22, 105, 93, 5, 46,
            98,
        ],
    );
}
