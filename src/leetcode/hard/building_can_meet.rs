pub fn leftmost_building_queries<Q: AsRef<[i32]>>(heights: impl AsRef<[i32]>, queries: impl AsRef<[Q]>) -> Vec<i32> {
    let heights = heights.as_ref();
    let data = MaxTree::new(heights);
    queries
        .as_ref()
        .iter()
        .map(|q| {
            let a = q.as_ref()[0] as usize;
            let b = q.as_ref()[1] as usize;
            let h = heights[a].max(heights[b]);
            if a == b {
                return a as i32;
            } else if heights[a.min(b)] < heights[a.max(b)] {
                return a.max(b) as i32;
            }
            data.query(a.max(b), h).map_or(-1, |x| x as i32)
        })
        .collect()
}

struct MaxTree(Vec<i32>);

impl MaxTree {
    pub fn new(values: &[i32]) -> Self {
        let mut data = vec![0; values.len() * 4];

        fn fill(data: &mut [i32], i: usize, c: usize, v: &mut impl Iterator<Item = i32>) {
            if c == 1 {
                data[i] = v.next().unwrap();
            } else {
                let m = c / 2;
                let (l, r) = (i * 2 + 1, i * 2 + 2);
                fill(data, l, m, v);
                fill(data, r, c - m, v);
                data[i] = data[l].max(data[r]);
            }
        }
        fill(&mut data, 0, values.len(), &mut values.iter().copied());
        MaxTree(data)
    }

    pub fn query(&self, l: usize, v: i32) -> Option<usize> {
        fn search(data: &[i32], s: usize, c: usize, i: usize, p: usize, v: i32) -> Option<usize> {
            if c == 1 {
                (data[i] > v && s > p).then_some(s)
            } else if data[i] <= v || s + c - 1 <= p {
                None
            } else {
                let m = c / 2;
                let (l, r) = (i * 2 + 1, i * 2 + 2);
                search(data, s, m, l, p, v).or_else(|| search(data, s + m, c - m, r, p, v))
            }
        }
        search(&self.0, 0, self.0.len() / 4, 0, l, v)
    }
}

#[test]
fn example1() {
    assert_eq!(
        vec![2, 5, -1, 5, 2],
        leftmost_building_queries([6, 4, 8, 5, 2, 7], [[0, 1], [0, 3], [2, 4], [3, 4], [2, 2]])
    )
}

#[test]
fn example2() {
    assert_eq!(
        vec![7, 6, -1, 4, 6],
        leftmost_building_queries([5, 3, 8, 2, 6, 1, 4, 6], [[0, 7], [3, 5], [5, 2], [3, 0], [1, 6]])
    )
}

#[test]
fn wa1() {
    assert_eq!(
        vec![
            0, 1, 3, 3, 5, 5, 1, 1, -1, -1, -1, -1, 3, -1, 2, 3, 5, 5, 3, -1, 3, 3, -1, -1, 5, -1, 5, -1, 4, 5, 5, -1,
            5, -1, 5, 5
        ],
        leftmost_building_queries(
            [1, 2, 1, 2, 1, 2],
            [
                [0, 0],
                [0, 1],
                [0, 2],
                [0, 3],
                [0, 4],
                [0, 5],
                [1, 0],
                [1, 1],
                [1, 2],
                [1, 3],
                [1, 4],
                [1, 5],
                [2, 0],
                [2, 1],
                [2, 2],
                [2, 3],
                [2, 4],
                [2, 5],
                [3, 0],
                [3, 1],
                [3, 2],
                [3, 3],
                [3, 4],
                [3, 5],
                [4, 0],
                [4, 1],
                [4, 2],
                [4, 3],
                [4, 4],
                [4, 5],
                [5, 0],
                [5, 1],
                [5, 2],
                [5, 3],
                [5, 4],
                [5, 5]
            ]
        )
    )
}
