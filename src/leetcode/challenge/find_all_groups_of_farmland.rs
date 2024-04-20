pub fn find_farmland(land: impl IntoIterator<Item = impl AsRef<[i32]>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let mut cur: Vec<Vec<i32>> = vec![];
    for (i, row) in land.into_iter().enumerate() {
        let i = i as i32;
        let mut new = vec![];
        let mut it = cur.into_iter().peekable();
        let mut line = None::<Vec<i32>>;
        let mut restart = 0;
        for (j, cell) in row.as_ref().iter().enumerate() {
            let j = j as i32;
            if j < restart {
                continue;
            }
            let here = it.peek().map(|v| v[1] == j).filter(|&b| b).and_then(|_| it.next());
            match (cell, here, &mut line) {
                (1, Some(mut v), _) => {
                    v[2] = i;
                    restart = v[3] + 1;
                    new.push(v);
                }
                (_, Some(v), _) => {
                    new.extend(line.take());
                    res.push(v)
                }
                (1, None, Some(v)) => v[3] = j,
                (1, None, None) => line = Some(vec![i, j, i, j]),
                (_, None, Some(_)) => new.extend(line.take()),
                (_, None, None) => {}
            }
        }
        new.extend(line);
        cur = new;
    }
    res.extend(cur);
    return res;
}

#[test]
fn example1() {
    assert_eq!(
        find_farmland([[1, 0, 0], [0, 1, 1], [0, 1, 1]]),
        vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]]
    )
}

#[test]
fn example2() {
    assert_eq!(find_farmland([[1, 1], [1, 1]]), vec![vec![0, 0, 1, 1]])
}

#[test]
fn example3() {
    assert_eq!(find_farmland([[0i32]]), Vec::<Vec<i32>>::new())
}

#[test]
fn wa1() {
    assert_eq!(
        find_farmland([[0, 1], [1, 0]]),
        vec![vec![0, 1, 0, 1], vec![1, 0, 1, 0]]
    )
}

#[test]
fn wa2() {
    assert_eq!(
        find_farmland([[0, 1, 0], [1, 0, 1], [0, 1, 0]]),
        vec![vec![0, 1, 0, 1], vec![1, 0, 1, 0], vec![1, 2, 1, 2], vec![2, 1, 2, 1]]
    )
}
