// object Solution:
//     def nearestPalindromic(n: String): String =
//         val x = BigInt(n)
//         val m = n.length
//         val mirror = n.take((m + 1) / 2) + n.take(m / 2).reverse
//         val t = (m + 1) / 2
//         val isPaly = n.view == n.view.reverse

//         def jump(dig: Char, dd: Int): String =
//             val u = (0 until t).takeWhile(i => n(t - i - 1) == dig).length
//             if u == t && dig == '9'  then
//                 ("1".view ++ Iterator.fill(m - 1)('0') ++ "1".view).mkString
//             else if u == t - 1 && dig == '0' && n(0) == '1' && m > 1 then
//                 Iterator.fill(m - 1)('9').mkString
//             else
//                 val first = n.view.take(t - u - 1)
//                 val follow = Seq((n(t - u - 1).toInt + dd).toChar)
//                 val fillLen = u * 2 - (m % 2).min(u)
//                 val fillDig = ('9'.toInt - dig.toInt  + '0'.toInt).toChar
//                 val follow2 = if m % 2 == 0 || fillLen > 0 then follow else Seq()
//                 val fill =  (0 until fillLen).view.map(_ => fillDig)
//                 (first ++ follow ++ fill ++ follow2 ++ first.reverse).mkString

//         val incremented = jump('9', 1)
//         val decremented = jump('0', -1)
//         val choose = Seq(decremented, mirror, incremented).filter(_ != n)
//         val bn = BigInt(n)
//         choose.minBy(x => (BigInt(x) - bn).abs)

//     end nearestPalindromic
// end Solution

use std::cmp::Ordering;

pub fn nearest_palindromic(n: String) -> String {
    let x = n.into_bytes();

    let t = x.len() / 2;
    let palindrome = x.iter().take(t).cmp(x.iter().rev().take(t));
    let xstr: Str = Box::new(x.iter().copied());
    let elems = match palindrome {
        Ordering::Less => vec![(decrement(&x), false), (mirror(&x), false), (increment(&x), true)],
        Ordering::Equal => vec![(decrement(&x), false), (increment(&x), true)],
        Ordering::Greater => vec![(decrement(&x), false), (mirror(&x), true), (increment(&x), true)],
    };
    let Some((u, _)) = elems
        .into_iter()
        .map(|(seq, less)| {
            (
                seq.clone(),
                if less {
                    minus(xstr.clone(), seq)
                } else {
                    minus(seq, xstr.clone())
                },
            )
        })
        .min_by(|(_, d1), (_, d2)| cmp(d1.clone(), d2.clone()))
    else {
        return <_>::default();
    };
    String::from_utf8(u.collect()).unwrap_or_default()
}
type Str<'a> = Box<dyn Seq<'a>>;
type Rev<'a> = Box<dyn Iterator<Item = u8> + 'a>;

trait Seq<'a>: DoubleEndedIterator<Item = u8> + 'a {
    fn clone_seq(&self) -> Box<dyn Seq<'a>>;
}
impl<'a, T: DoubleEndedIterator<Item = u8> + Clone + 'a> Seq<'a> for T {
    fn clone_seq(&self) -> Box<dyn Seq<'a>> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Seq<'_>> {
    fn clone(&self) -> Self {
        self.clone_seq()
    }
}
fn jump(xs: &[u8], dig: u8, dd: i8) -> Str {
    let m = xs.len();
    let t = (m + 1) / 2;
    let u = (0..t).take_while(|i| xs[t - i - 1] == dig).count();
    if u == t && dig == b'9' {
        return Box::new([b'1'].into_iter().chain((0..m - 1).map(|_| b'0')).chain([b'1']));
    } else if u == t - 1 && dig == b'0' && xs[0] == b'1' && m > 1 {
        return Box::new((0..m - 1).map(|_| b'9'));
    }
    let first = xs.iter().take(t - u - 1).copied();
    let follow = [(xs[t - u - 1] as i8 + dd) as u8];
    let fill_len = u * 2 - (m % 2).min(u);
    let fill_dig = b'9' - dig + b'0';
    let follow2 = (m % 2 == 0 || fill_len > 0).then_some(follow[0]);
    let fill = (0..fill_len).map(move |_| fill_dig);

    let full = (first.clone())
        .chain(follow)
        .chain(fill)
        .chain(follow2)
        .chain(first.rev());
    Box::new(full)
}

fn increment(xs: &[u8]) -> Str {
    jump(xs, b'9', 1)
}
fn decrement(xs: &[u8]) -> Str {
    jump(xs, b'0', -1)
}
fn mirror(xs: &[u8]) -> Str {
    let m = xs.len();
    let first = xs.iter().take((m + 1) / 2);
    let second = xs.iter().take(m / 2).rev();
    Box::new(first.chain(second).copied())
}

fn minus<'a>(x: Str<'a>, y: Str<'a>) -> impl Iterator<Item = u8> + Clone + 'a {
    let x = x.chain([b'0']).rev();
    let y = y.chain([b'0']).rev();
    x.zip(y).scan(0, |c, (a, b)| {
        let d = a as i32 - b as i32 - *c;
        let add = (d < 0) as i32;
        *c = add;
        Some((d + add * 10) as u8)
    })
}

fn cmp(x: impl Iterator<Item = u8>, y: impl Iterator<Item = u8>) -> Ordering {
    x.zip(y).fold(Ordering::Equal, |acc, (a, b)| a.cmp(&b).then(acc))
}
