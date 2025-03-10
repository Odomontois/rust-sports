pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    let mut search = Search { n, data: vec![0; 2 * n as usize - 1] };
    search.search(0, 0);
    search.data
}

struct Search {
    data: Vec<i32>,
    n: i32,
}
impl Search {
    fn search(&mut self, mut i: usize, seen: u32) -> Option<()> {
        while i < self.data.len() && self.data[i] != 0 {
            i += 1;
        }
        if i == self.data.len() {
            return None;
        }
        for x in (1..=self.n).rev() {
            if seen & (1 << x) != 0 {
                continue;
            }
            self.data[i] = x;
            let j = i + x as usize;
            if x > 1 {
                if j < self.data.len() && self.data[j] == 0 {
                    self.data[j] = x;
                } else {
                    continue;
                }
            }
            let () = self.search(i + 1, seen | (1 << x))?;
            if x > 1 {
                self.data[j] = 0;
            }
        }
        self.data[i] = 0;
        Some(())
    }
}

#[test]
fn example1() {
    assert_eq!(construct_distanced_sequence(3), vec![3, 1, 2, 3, 2])
}

#[test]
fn example2() {
    assert_eq!(construct_distanced_sequence(5), vec![5, 3, 1, 4, 3, 5, 2, 4, 2])
}
