use self::SuffixType::*;

pub fn last_substring(s: String) -> String {
    s[find_last(s.as_bytes())..].to_string()
}

fn find_last<A: Suffix>(xs: &[A]) -> usize {
    *suffix_array(xs).last().unwrap()
}

// Created base on https://zork.net/~st/jottings/sais.html
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum SuffixType {
    S,
    L,
}

trait Suffix: Ord {
    const TOTAL_SIZE: usize;
    fn order(&self) -> usize;
}
#[derive(Debug, Clone)]
struct Summary {
    summary: Vec<usize>,
    size: usize,
    offsets: Vec<usize>,
}

struct SuffixArray<'a, A, D> {
    data: &'a [A],
    size: usize,
    buckets: Vec<usize>,
    type_map: Vec<SuffixType>,
    debug: D,
}

impl<'a, A: Suffix, D: Debugger> SuffixArray<'a, A, D> {
    fn build_type_map(data: &'a [A]) -> Vec<SuffixType> {
        let n = data.len();
        let mut res = vec![S; n + 1];
        if data.is_empty() {
            return res;
        }

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

    fn find_bucket_sizes(data: &'a [A], size: usize) -> Vec<usize> {
        let mut res = vec![0; size];
        for a in data {
            res[a.order()] += 1
        }
        res
    }

    pub fn make(data: &'a [A], size: usize, debug: D) -> Self {
        Self {
            data,
            size,
            buckets: Self::find_bucket_sizes(data, size),
            type_map: Self::build_type_map(data),
            debug,
        }
    }
    fn lms_substrings_are_equal(&self, offset_a: usize, offset_b: usize) -> bool {
        if offset_a == self.data.len() || offset_b == self.data.len() {
            return false;
        }

        for i in 0.. {
            let a_is_lms = self.is_lms_char(i + offset_a);
            let b_is_lms = self.is_lms_char(i + offset_b);

            if i > 0 && a_is_lms && b_is_lms {
                return true;
            }
            if a_is_lms != b_is_lms {
                return false;
            }
            if self.data[i + offset_a] != self.data[i + offset_b] {
                return false;
            }
        }
        false
    }

    fn is_lms_char(&self, offset: usize) -> bool {
        offset != 0 && self.type_map[offset] == S && self.type_map[offset - 1] == L
    }

    fn find_bucket_heads(&self) -> Vec<usize> {
        self.buckets
            .iter()
            .scan(1, |pos, &c| {
                let x = *pos;
                *pos += c;
                Some(x)
            })
            .collect()
    }

    fn find_bucket_tails(&self) -> Vec<usize> {
        self.buckets
            .iter()
            .scan(0, |pos, &c| {
                *pos += c;
                Some(*pos)
            })
            .collect()
    }

    fn guess_lms_sort(&self) -> Vec<Option<usize>> {
        let mut guessed = vec![None; self.data.len() + 1];

        let mut bucket_tails = self.find_bucket_tails();
        for (i, key) in self.data.iter().enumerate() {
            if !self.is_lms_char(i) {
                continue;
            }
            let ord = key.order();
            guessed[bucket_tails[ord]] = Some(i);
            bucket_tails[ord] -= 1;
            self.debug.show_suffix_array(&guessed, None);
        }
        guessed[0] = Some(self.data.len());
        self.debug.show_suffix_array(&guessed, None);
        guessed
    }

    fn induce_sort_l(&self, guess: &mut [Option<usize>]) {
        let mut heads = self.find_bucket_heads();
        for i in 0..guess.len() {
            let j = match guess[i] {
                Some(x) if x > 0 && self.type_map[x - 1] == L => x - 1,
                _ => continue,
            };
            let key = self.data[j].order();
            guess[heads[key]] = Some(j);
            heads[key] += 1;
            self.debug.show_suffix_array(guess, Some(i));
        }
    }

    fn induce_sort_s(&self, guess: &mut [Option<usize>]) {
        let mut tails = self.find_bucket_tails();
        for i in (0..guess.len()).rev() {
            let j = match guess[i] {
                Some(x) if x > 0 && self.type_map[x - 1] == S => x - 1,
                _ => continue,
            };
            let key = self.data[j].order();
            guess[tails[key]] = Some(j);
            tails[key] -= 1;
            self.debug.show_suffix_array(guess, Some(i));
        }
    }

    fn summarize(&self, guessed: &mut [usize]) -> Summary {
        let mut lms_names = vec![None; self.data.len() + 1];
        let mut name = 0;
        let mut last_offset = guessed[0];
        lms_names[guessed[0]] = Some(name);
        self.debug.show_suffix_array(&lms_names, None);
        for &off in guessed.iter().skip(1) {
            if !self.is_lms_char(off) {
                continue;
            }
            if !self.lms_substrings_are_equal(last_offset, off) {
                name += 1
            }
            last_offset = off;
            lms_names[off] = Some(name);
            self.debug.show_suffix_array(&lms_names, None);
        }

        let (offsets, summary) = lms_names
            .into_iter()
            .enumerate()
            .filter_map(|(i, no)| no.map(|n| (i, n)))
            .unzip();

        Summary {
            size: name + 1,
            summary,
            offsets,
        }
    }

    fn accurate_lms_sort(&self, summary: &Summary) -> Vec<Option<usize>> {
        let sum_suff = self.summary_suffix_array(summary);

        let mut res = vec![None; self.data.len() + 1];
        let mut tails = self.find_bucket_tails();
        for &sumi in sum_suff[2..].iter().rev() {
            let data_index = summary.offsets[sumi];
            let bucket_index = self.data[data_index].order();
            res[tails[bucket_index]] = Some(data_index);
            tails[bucket_index] -= 1;
            self.debug.show_suffix_array(&res, None);
        }
        res[0] = Some(self.data.len());
        self.debug.show_suffix_array(&res, None);
        res
    }

    fn summary_suffix_array(&self, summary: &Summary) -> Vec<usize> {
        if summary.size == summary.summary.len() {
            let mut res = vec![0; summary.summary.len() + 1];
            res[0] = summary.summary.len();
            for (x, &y) in summary.summary.iter().enumerate() {
                res[y + 1] = x
            }
            res
        } else {
            SuffixArray::make(&summary.summary, summary.size, self.debug).suffix_array_is_sa()
        }
    }

    fn suffix_array_is_sa(&self) -> Vec<usize> {
        let mut guessed = self.guess_lms_sort();
        self.induce_sort_l(&mut guessed);
        self.induce_sort_s(&mut guessed);
        let mut guessed: Vec<_> = guessed.into_iter().flatten().collect();

        let summary = self.summarize(&mut guessed);
        let mut result = self.accurate_lms_sort(&summary);
        self.induce_sort_l(&mut result);
        self.induce_sort_s(&mut result);

        result.into_iter().flatten().collect()
    }
}

fn suffix_array<A: Suffix>(data: &[A]) -> Vec<usize> {
    SuffixArray::make(data, A::TOTAL_SIZE, Runtime).suffix_array_is_sa()
}

impl<A: Suffix> Suffix for Option<A> {
    const TOTAL_SIZE: usize = A::TOTAL_SIZE + 1;

    fn order(&self) -> usize {
        self.as_ref().map_or(0, |x| x.order() + 1)
    }
}

impl Suffix for u8 {
    fn order(&self) -> usize {
        *self as usize
    }

    const TOTAL_SIZE: usize = 256;
}

impl Suffix for usize {
    fn order(&self) -> usize {
        *self
    }

    const TOTAL_SIZE: usize = std::usize::MAX;
}

trait Debugger: Copy {
    fn show_suffix_array(&self, arr: &[Option<usize>], pos: Option<usize>);
}

#[derive(Clone, Copy)]
struct Runtime;
impl Debugger for Runtime {
    fn show_suffix_array(&self, _: &[Option<usize>], _: Option<usize>) {}
}
