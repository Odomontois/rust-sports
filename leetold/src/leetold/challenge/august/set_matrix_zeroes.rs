pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (n, m) = (matrix.len(), matrix[0].len());

    let first_row = (0..n).any(|i| matrix[i][0] == 0);
    let first_col = (0..m).any(|j| matrix[0][j] == 0);

    for i in 1..n {
        for j in 1..m {
            if matrix[i][j] == 0 {
                matrix[0][i] = 0;
                matrix[i][0] = 0
            }
        }
    }

    for i in 1..n {
        for j in 1..m {
            if matrix[0][j] == 0 || matrix[i][0] == 0 {
                matrix[i][j] = 0
            }
        }
    }

    if first_row {
        (0..n).for_each(|i| matrix[i][0] = 0)
    }
    if first_col {
        (0..m).for_each(|j| matrix[0][j] = 0)
    }
}
