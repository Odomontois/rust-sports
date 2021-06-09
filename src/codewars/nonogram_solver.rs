use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Debug;
use lazy_static::lazy_static;

fn solve_nonogram((top_clues, left_clues): ([&[u8]; 5], [&[u8]; 5])) -> [[u8; 5]; 5] {
    for &x0 in SOL_MAP[left_clues[0]] {
        for &x1 in SOL_MAP[left_clues[1]] {
            for &x2 in SOL_MAP[left_clues[2]] {
                for &x3 in SOL_MAP[left_clues[3]] {
                    for &x4 in SOL_MAP[left_clues[4]] {
                        let x = [x0, x1, x2, x3, x4];
                        if pattern(transpose(x)) == top_clues {
                            return bits(x);
                        }
                    }
                }
            }
        }
    }
    panic!("No solution")
}

fn map_arr<A, B: Debug, F, const N: usize>(v: [A; N], f: F) -> [B; N]
where
    F: FnMut(&A) -> B,
{
    v.iter().map(f).collect::<Vec<_>>().try_into().unwrap()
}

fn bits<const N: usize>(map: [u8; N]) -> [[u8; N]; N] {
    let bw = (0..N).rev().collect::<Vec<_>>().try_into().unwrap();
    map_arr(map, |&x| map_arr(bw, |i| (x & (1 << i)) >> i))
}

fn transpose<const N: usize>(map: [u8; N]) -> [u8; N] {
    let mut res = [0; N];
    for i in 0..N {
        for j in 0..N {
            let b = (map[i] & (1 << j)) >> j;
            res[4 - j] |= b << (4 - i);
        }
    }
    res
}

fn pattern<const N: usize>(map: [u8; N]) -> [&'static [u8]; N] {
    map_arr(map, |u| SOL_REV[u])
}

const SOLS: &[(&[u8], &[u8])] = &[
    (&[], &[0b00000]),
    (&[1], &[0b10000, 0b01000, 0b00100, 0b00010, 0b00001]),
    (&[2], &[0b11000, 0b01100, 0b00110, 0b00011]),
    (&[3], &[0b11100, 0b01110, 0b00111]),
    (&[4], &[0b11110, 0b01111]),
    (&[5], &[0b11111]),
    (&[1, 1], &[0b10100, 0b10010, 0b10001, 0b01010, 0b01001, 0b00101]),
    (&[1, 2], &[0b10110, 0b10011, 0b01011]),
    (&[1, 3], &[0b10111]),
    (&[2, 1], &[0b11010, 0b11001, 0b01101]),
    (&[2, 2], &[0b11011]),
    (&[3, 1], &[0b11101]),
    (&[1, 1, 1], &[0b10101]),
];

lazy_static! {
    static ref SOL_MAP: HashMap<&'static [u8], &'static [u8]> = SOLS.iter().copied().collect();
    static ref SOL_REV: HashMap<u8, &'static [u8]> = SOLS
        .iter()
        .copied()
        .flat_map(|(k, vs)| { vs.iter().map(move |&v| (v, k)) })
        .collect();
}

#[cfg(test)]
mod basic_tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(solve_nonogram(CLUES_1), ANS_1);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_nonogram(CLUES_2), ANS_2);
    }

    const CLUES_1: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1, 1], &[4], &[1, 1, 1], &[3], &[1]],
        [&[1], &[2], &[3], &[2, 1], &[4]],
    );

    const ANS_1: [[u8; 5]; 5] = [
        [0, 0, 1, 0, 0],
        [1, 1, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 1, 0, 1, 0],
        [0, 1, 1, 1, 1],
    ];

    const CLUES_2: ([&[u8]; 5], [&[u8]; 5]) = (
        [&[1], &[3], &[1], &[3, 1], &[3, 1]],
        [&[3], &[2], &[2, 2], &[1], &[1, 2]],
    );

    const ANS_2: [[u8; 5]; 5] = [
        [0, 0, 1, 1, 1],
        [0, 0, 0, 1, 1],
        [1, 1, 0, 1, 1],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 1, 1],
    ];
}
