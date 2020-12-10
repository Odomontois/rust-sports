// Created base on https://zork.net/~st/jottings/sais.html
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum SuffixType { S, L }

use SuffixType::*;

/// Builds a map marking each suffix of the data as S_TYPE or L_TYPE.
fn build_type_map<A: Ord>(data: &[A]) -> Vec<SuffixType> {
    let n = data.len();
    let mut res = vec![S; n + 1];
    if data.is_empty() { return res; }

    res[n - 1] = L;

    for i in (0..n - 1).rev() {
        res[i] = if data[i] > data[i + 1] {
            L
        } else if data[i] == data[i + 1] && res[i + 1] == L {
            L
        } else {
            S
        }
    }


    res
}

/// Returns true if the character at offset is a left-most S-type.
fn is_lms_char(offset: usize, typemap: &[SuffixType]) -> bool {
    offset != 0 && typemap[offset] == S && typemap[offset - 1] == L
}

// fn lms_substrings_are_equal<A: Ord>(data: &[A], typemap: &[SuffixType], offset_a: usize, offset_b: usize) {}

mod test {
    #[allow(unused_imports)]
    use super::*;

    fn join(x: impl Iterator<Item=String>) -> String {
        x.collect::<Vec<_>>().join("")
    }

    fn show_type_map(data: &str) {
        let typemap = build_type_map(&data.chars().collect::<Vec<_>>());
        let tmstr = join(typemap.iter().map(|c| format!("{:?}", c)));
        let lmstr = join((0..typemap.len()).map(|o| (if is_lms_char(o, typemap.as_slice()) { "^" } else { " " }).to_string()));
        println!("{}\n{}\n{}", data, tmstr, lmstr);
    }

    #[test]
    fn test_type_map() {
        show_type_map("caabbage")
    }
}


