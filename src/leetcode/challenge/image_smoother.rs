pub fn image_smoother(mut img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = img.len();
    let m = img[0].len();
    (0..n).for_each(|i| sum3(&mut img, (0..m).map(|j| (i, j))));
    (0..m).for_each(|j| sum3(&mut img, (0..n).map(|i| (i, j))));
    let dim = |i, n| 3 - (n == 1) as i32 - (i == 0 || i == n - 1) as i32;
    fn sum3(vec: &mut Vec<Vec<i32>>, it: impl Iterator<Item = (usize, usize)>) {
        let mut it = it.peekable();
        let mut prev = 0;
        while let Some((i, j)) = it.next() {
            let v = vec[i][j];
            vec[i][j] += prev;
            prev = v;
            if let Some(&(i1, j1)) = it.peek() {
                vec[i][j] += vec[i1][j1];
            }
        }
    }

    (0..n).for_each(|i| {
        (0..m).for_each(|j| img[i][j] /= dim(i, n) * dim(j, m));
    });
    img
}
