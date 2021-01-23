// Created base on https://zork.net/~st/jottings/sais.html
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum SuffixType { S, L }

use SuffixType::*;
use std::collections::BTreeMap;

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

fn lms_substrings_are_equal<A: Ord>(data: &[A], typemap: &[SuffixType], offset_a: usize, offset_b: usize) -> bool {
    if offset_a == data.len() || offset_b == data.len() { return false; }

    for i in 0.. {
        let a_is_lms = is_lms_char(i + offset_a, typemap);
        let b_is_lms = is_lms_char(i + offset_b, typemap);

        if i > 0 && a_is_lms && b_is_lms { return true; }
        if a_is_lms != b_is_lms { return false; }
        if data[i + offset_a] != data[i + offset_b] { return false; }
    }
    return false;
}

type Buckets<'a, A> = BTreeMap<&'a A, usize>;

fn find_bucket_sizes<A: Ord>(data: &[A]) -> Buckets<A> {
    let mut res = BTreeMap::new();
    for a in data {
        *res.entry(a).or_insert(0) += 1
    }
    res
}

fn find_bucket_heads<'a, 'b : 'a, A: Ord + 'a>(bucket_sizes: &'b Buckets<'a, A>) -> Buckets<'a, A> {
    bucket_sizes.iter().scan(1, |pos, (&elem, &c)| {
        let x = *pos;
        *pos += c;
        Some((elem, x))
    }).collect()
}

fn find_bucket_tails<'a, 'b : 'a, A: Ord + 'a>(bucket_sizes: &'b Buckets<'a, A>) -> Buckets<'a, A> {
    bucket_sizes.iter().scan(0, |pos, (&elem, &c)| {
        *pos += c;
        Some((elem, *pos))
    }).collect()
}


fn guess_lms_sort<A: Ord>(data: &[A], buckets: &Buckets<A>, type_map: &[SuffixType]) -> Vec<Option<usize>> {
    let mut guessed = vec![None; data.len() + 1];

    let mut bucket_tails = find_bucket_tails(&buckets);
    for (i, key) in data.iter().enumerate() {
        if !is_lms_char(i, type_map) { continue; }
        guessed[bucket_tails[key]] = Some(i);
        bucket_tails.entry(key).and_modify(|v| *v -= 1);
        show_suffix_array(&guessed, None);
    }
    guessed[0] = Some(data.len());
    show_suffix_array(&guessed, None);
    guessed
}

fn induce_sort_l<A: Ord>(data: &[A], guess: &mut [Option<usize>], buckets: &Buckets<A>, type_map: &[SuffixType]) {
    let mut heads = find_bucket_heads(buckets);
    for i in 0..guess.len() {
        let j = match guess[i] {
            Some(x) if x > 0 && type_map[x - 1] == L => x - 1,
            _ => continue
        };
        let key = &data[j];
        guess[heads[key]] = Some(j);
        heads.entry(key).and_modify(|x| *x += 1);
        show_suffix_array(guess, Some(i));
    }
}

fn induce_sort_s<A: Ord>(data: &[A], guess: &mut [Option<usize>], buckets: &Buckets<A>, type_map: &[SuffixType]) {
    let mut tails = find_bucket_tails(buckets);
    for i in (0..guess.len()).rev() {
        let j = match guess[i] {
            Some(x) if x > 0 && type_map[x - 1] == S => x - 1,
            _ => continue,
        };
        let key = &data[j];
        guess[tails[key]] = Some(j);
        tails.entry(key).and_modify(|x| *x -= 1);
        show_suffix_array(guess, Some(i));
    }
}

fn show_suffix_array(arr: &[Option<usize>], pos: Option<usize>) {
    println!("{}",
             arr.iter()
                 .map(|ox| ox.map(|x| format!("{:02}", x)).unwrap_or("--".to_string()))
                 .collect::<Vec<_>>().join(" ")
    );
    for p in pos {
        println!("{}",
                 (0..arr.len())
                     .map(|i| if i == p { "^^" } else { "  " })
                     .collect::<Vec<_>>().join(" ")
        );
    }
}

#[derive(Debug, Clone)]
struct Summary { summary: Vec<usize>, size: usize, offsets: Vec<usize> }

fn summarize<A: Ord>(data: &[A], guessed: &mut [usize], type_map: &[SuffixType]) -> Summary {
    let mut lms_names = vec![None; data.len() + 1];
    let mut name = 0;
    let mut last_offset = guessed[0];
    lms_names[guessed[0]] = Some(name);
    show_suffix_array(&lms_names, None);
    for &off in guessed.iter().skip(1) {
        if !is_lms_char(off, type_map) { continue; }
        if !lms_substrings_are_equal(data, type_map, last_offset, off) {
            name += 1
        }
        last_offset = off;
        lms_names[off] = Some(name);
        show_suffix_array(&lms_names, None);
    }

    let (offsets, summary) =
        lms_names.into_iter().enumerate()
            .filter_map(|(i, no)| no.map(|n| (i, n)))
            .unzip();

    Summary { size: name + 1, summary, offsets }
}

// fn suffix_array_is_sa<A: Ord>(data: &[A]) {
//     let type_map = build_type_map(data);
//     let buckets = find_bucket_sizes(data);
//     let mut guessed = guess_lms_sort(data, &buckets, &type_map);
//     induce_sort_l(data, &mut guessed, &buckets, &type_map);
//     induce_sort_s(data, &mut guessed, &buckets, &type_map);
//     let mut guessed: Vec<_> = guessed.into_iter().flatten().collect();
//     let sum = summarize(&data, &mut guessed, &type_map);
//
//     unimplemented!()
// }


mod test {
    #[allow(unused_imports)]
    use super::*;

    fn join(x: impl Iterator<Item=String>) -> String {
        x.collect::<Vec<_>>().join("")
    }

    fn chars(s: &str) -> Vec<char> { s.chars().collect() }

    fn show_type_map(data: &str) {
        let typemap = build_type_map(&data.chars().collect::<Vec<_>>());
        let tmstr = join(typemap.iter().map(|c| format!("{:?}", c)));
        let lmstr = join((0..typemap.len()).map(|o| (if is_lms_char(o, typemap.as_slice()) { "^" } else { " " }).to_string()));
        println!("{}\n{}\n{}", data, tmstr, lmstr);
    }


    #[test]
    fn test_type_map() {
        show_type_map("caabbage");
        show_type_map("rikki-tikki-tikka");
    }

    #[test]
    fn rikki_test() {
        let s = "rikki-tikki-tikka".as_bytes();
        let tm = build_type_map(s);
        println!("{}", lms_substrings_are_equal(s, &tm, 1, 7));
        println!("{}", lms_substrings_are_equal(s, &tm, 1, 13));
    }

    #[test]
    fn bucket_test() {
        let cab_c: Vec<_> = "cabbage".chars().collect();
        let cabb = find_bucket_sizes(&cab_c);
        println!("{:?}", find_bucket_heads(&cabb));
        println!("{:?}", find_bucket_tails(&cabb));
    }

    #[test]
    fn show_suf_test() {
        show_suffix_array(&[Some(2), None, Some(4)], None);
        show_suffix_array(&[Some(2), None, Some(4)], Some(2));
    }


    fn guess_test(word: &str) {
        let data = word.as_bytes();
        let buckets = find_bucket_sizes(data);
        let types = build_type_map(data);
        show_type_map(word);
        let mut guess = guess_lms_sort(data, &buckets, &types);
        induce_sort_l(data, &mut guess, &buckets, &types);
        induce_sort_s(data, &mut guess, &buckets, &types);
        let mut guess: Vec<_> = guess.into_iter().flatten().collect();
        let summ = summarize(data, &mut guess, &types);
        println!("{:?}", summ);
    }

    #[test]
    fn cabbage_guess_test() { guess_test("cabbage") }

    #[test]
    fn baa_guess_test() { guess_test("baabaabac") }
}


