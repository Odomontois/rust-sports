struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        static A: u8 = 'a' as u8;
        let mut qs = [[0; 26]; 2];
        let mut seen = [[false; 26]; 2];
        let mut calc = |xs: Vec<u8>, i: usize| {
            for x in xs {
                qs[i][(x - A) as usize] += 1;
            }
            for j in 0..26 { seen[i][j] = qs[i][j] != 0 }
            qs[i].sort();
        };
        calc(word1.into_bytes(), 0);
        calc(word2.into_bytes(), 1);
        seen[0] == seen[1] && qs[0] == qs[1]
    }
}

pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (n, m) = (mat.len(), mat[0].len());
    let mut d = vec![vec![]; n + m - 1];
    for i in 0..n {
        for j in 0..m {
            d[i + m - 1 - j].push(mat[i][j])
        }
    }
    for v in &mut d {
        v.sort_by_key(|&x| std::cmp::Reverse(x))
    }
    for i in 0..n {
        for j in 0..m {
            for x in d[i + m - 1 - j].pop() {
                mat[i][j] = x;
            }
        }
    }
    mat
}





