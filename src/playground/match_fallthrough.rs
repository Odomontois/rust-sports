#[allow(unused)]
fn original() {
    let [mut k1, c1, c2, mut h1, len] = [0; 5];
    let tail = [0; 3];
    'outer: {
        'case1: {
            'case2: {
                'case3: {
                    match len & 3 {
                        3 => break 'case3,
                        2 => break 'case2,
                        1 => break 'case1,
                        _ => break 'outer,
                    }
                } // 'case3:
                k1 ^= (tail[2] as u32) << 16;
            } // 'case2:
            k1 ^= (tail[1] as u32) << 8;
        } // case1:
        k1 ^= tail[0] as u32;
        k1 *= c1;
        k1 = k1.rotate_left(15);
        k1 *= c2;
        h1 ^= k1;
    }
}

struct HashState {
    k1: u32,
    c1: u32,
    c2: u32,
    h1: u32,
    len: u32,
    tail: [u8; 3],
}

impl HashState {
    fn case1(&mut self) {
        self.k1 ^= self.tail[0] as u32;
        self.k1 *= self.c1;
        self.k1 = self.k1.rotate_left(15);
        self.k1 *= self.c2;
        self.h1 ^= self.k1;
    }
    fn case2(&mut self) {
        self.k1 ^= (self.tail[1] as u32) << 8;
        self.case1();
    }
    fn case3(&mut self) {
        self.k1 ^= (self.tail[2] as u32) << 16;
        self.case2();
    }
    fn original(&mut self) {
        match self.len & 3 {
            3 => self.case3(),
            2 => self.case2(),
            1 => self.case1(),
            _ => {}
        }
    }
}
