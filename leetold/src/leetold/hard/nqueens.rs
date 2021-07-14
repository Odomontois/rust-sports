use std::iter::{once, repeat};

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    Queens::new(n as u8)
        .map(|p| p.into_iter().map(|x| draw_line(x as usize, n as usize)).collect())
        .collect()
}

pub fn total_n_queens(n: i32) -> i32 {
    Queens::new(n as u8).count() as i32
}

#[derive(Debug, Clone, Default)]
struct Queens {
    pub size: u8,
    pos: Vec<u8>,
    column: u16,
    main: u32,
    secondary: u32,
}

impl Queens {
    fn new(size: u8) -> Self {
        Self {
            size: size,
            ..Self::default()
        }
    }
    fn mark(&mut self, line: u8, column: u8) {
        if column == 0 {
            return;
        }
        self.column ^= 1 << column;
        self.main ^= 1 << (column + self.size - line);
        self.secondary ^= 1 << (column + line);
    }
    fn available(&mut self, line: u8, column: u8) -> bool {
        self.column & (1 << column) == 0
            && self.main & (1 << column + self.size - line) == 0
            && self.secondary & (1 << column + line) == 0
    }
    fn search(&mut self, line: u8) -> bool {
        let current = self.pos[line as usize];
        self.mark(line, current);
        for column in current + 1..=self.size {
            if self.available(line, column) {
                self.mark(line, column);
                self.pos[line as usize] = column;
                return true;
            }
        }
        self.pos[line as usize] = 0;
        false
    }
}

impl Iterator for Queens {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cur = if self.pos.len() < self.size as usize {
            self.pos = vec![0; self.size as usize];
            0
        } else {
            self.size - 1
        };
        loop {
            if self.search(cur) {
                cur += 1;
                if cur == self.size {
                    return Some(self.pos.iter().map(|&x| x - 1).collect());
                }
            } else {
                if cur == 0 {
                    return None;
                }
                cur -= 1;
            }
        }
    }
}

fn draw_line(pos: usize, size: usize) -> String {
    repeat('.')
        .take(pos)
        .chain(once('Q'))
        .chain(repeat('.').take(size - pos - 1))
        .collect()
}

#[test]
fn check() {
    for x in solve_n_queens(5) {
        for s in x {
            println!("{}", s)
        }
        println!("-----------------")
    }
}
