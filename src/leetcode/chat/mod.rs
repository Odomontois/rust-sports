use std::ops::Index;

#[derive(Debug)]
struct NumMatrix { sums: Vec<Vec<i32>> }

impl NumMatrix {
    #[allow(dead_code)]
    fn new(mut sums: Vec<Vec<i32>>) -> Self {
        let n = sums.len();
        let m = sums.get(0).map(|v| v.len()).unwrap_or(0);
        for i in 1..n {
            for j in 0..m {
                sums[i][j] += sums[i - 1][j]
            }
        }
        for i in 0..n {
            for j in 1..m {
                sums[i][j] += sums[i][j - 1]
            }
        }
        NumMatrix { sums }
    }


    #[allow(dead_code)]
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (r1, c1, r2, c2) = (row1 as usize, col1 as usize, row2 as usize + 1, col2 as usize + 1);
        self[(r2, c2)] - self[(r2, c1)] - self[(r1, c2)] + self[(r1, c1)]
    }
}

impl Index<(usize, usize)> for NumMatrix {
    type Output = i32;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        if i == 0 || j == 0 { &0 } else { &self.sums[i - 1][j - 1] }
    }
}

// #[test]
// fn matr_check() {
//     let m = NumMatrix::new(vec![vec![1; 3]; 5]);
//     let mrx = [
//         [3, 0, 1, 4, 2],
//         [5, 6, 3, 2, 1],
//         [1, 2, 0, 1, 5],
//         [4, 1, 0, 1, 7],
//         [1, 0, 3, 0, 5]
//     ];
//     let nm = NumMatrix::new(mrx.map(|v| v.to_vec()).to_vec());
//     println!("{:?} {}", m, m.sum_region(1, 1, 4, 2));
//     println!("{:#?}", nm);
//     println!("{:#?}", nm.sum_region(2, 1, 4, 3));
//     println!("{:#?}", nm.sum_region(1, 1, 2, 2));
//     println!("{:#?}", nm.sum_region(1, 2, 2, 4));
// }
