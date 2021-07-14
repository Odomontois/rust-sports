//1595. Minimum Cost to Connect Two Groups of Points

pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
    Connect::new(cost).calc()
}

struct Connect {
    cache: Vec<Vec<i32>>,
    cost: Vec<Vec<i32>>,
    min: Vec<i32>,
    n: usize,
    m: usize,
}

impl Connect {
    fn new(cost: Vec<Vec<i32>>) -> Self {
        let n = cost.len();
        let m = cost.first().map(|x| x.len()).unwrap_or(0);
        let min = (0..m).map(|j| (0..n).map(|i| cost[i][j]).min().unwrap_or(0)).collect();
        let cache = vec![vec![-1; 1 << m]; n + 1];

        Self { cache, min, cost, m, n }
    }

    fn idxs(x: usize, limit: usize) -> impl Iterator<Item = usize> {
        (0..limit).filter(move |&i| x & (1 << i) != 0)
    }

    pub fn calc(&mut self) -> i32 {
        self.recur(0, (1 << self.m) - 1)
    }

    fn recur(&mut self, i: usize, y: usize) -> i32 {
        let mut result = self.cache[i][y];
        if result >= 0 {
            return result;
        }
        result = if i == self.n {
            Self::idxs(y, self.m).map(|i| self.min[i]).sum()
        } else {
            (0..self.m)
                .map(|j| self.cost[i][j] + self.recur(i + 1, y & !(1 << j)))
                .min()
                .unwrap_or(0)
        };
        self.cache[i][y] = result;
        result
    }
}

#[test]
fn test() {
    fn check<const N: usize>(xs: &[[i32; N]], exp: i32) {
        assert_eq!(connect_two_groups(xs.iter().map(|v| v.to_vec()).collect()), exp);
    }
    check(&[[15, 96], [36, 2]], 17);
    check(&[[1, 3, 5], [4, 1, 1], [1, 5, 3]], 4);
    check(&[[2, 5, 1], [3, 4, 7], [8, 1, 2], [6, 2, 4], [3, 8, 8]], 10);
    check(
        &[
            [32, 11, 79, 32, 75, 39, 27, 67],
            [53, 8, 1, 70, 42, 68, 79, 92],
            [89, 0, 64, 57, 4, 15, 55, 59],
            [68, 4, 75, 29, 5, 20, 89, 95],
            [70, 82, 44, 6, 63, 41, 92, 67],
            [23, 96, 34, 13, 98, 72, 92, 35],
            [93, 9, 63, 42, 65, 47, 50, 38],
            [86, 89, 5, 32, 55, 53, 29, 20],
            [77, 33, 79, 64, 0, 44, 82, 6],
            [69, 55, 18, 12, 89, 54, 97, 10],
            [56, 91, 30, 2, 30, 83, 67, 60],
        ],
        102,
    );
    check(
        &[
            [11, 18, 21, 37, 45, 77, 45, 19, 82, 97],
            [88, 56, 71, 64, 6, 3, 39, 40, 73, 30],
            [45, 66, 40, 55, 47, 66, 11, 25, 89, 53],
            [65, 64, 73, 14, 20, 77, 37, 34, 3, 10],
            [31, 20, 74, 29, 65, 39, 66, 7, 11, 27],
            [44, 50, 68, 35, 85, 43, 29, 90, 29, 95],
            [75, 35, 79, 28, 33, 32, 47, 63, 94, 48],
            [47, 73, 43, 3, 97, 52, 83, 90, 66, 8],
            [84, 6, 7, 75, 46, 83, 7, 88, 96, 81],
            [76, 21, 78, 34, 26, 23, 46, 55, 90, 58],
            [97, 50, 26, 19, 89, 90, 5, 20, 13, 13],
        ],
        139,
    );
}
