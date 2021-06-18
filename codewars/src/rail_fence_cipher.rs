fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let mut elems = vec![vec![]; num_rails];
    for (c, i) in text.chars().zip(fence_levels(num_rails)) {
        elems[i].push(c);
    }
    elems.into_iter().flatten().collect()
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let mut idxs = vec![vec![]; num_rails];
    for (i, j) in (0..text.len()).zip(fence_levels(num_rails)) {
        idxs[j].push(i);
    }
    let mut res = vec![' '; text.len()];
    for (c, i) in text.chars().zip(idxs.into_iter().flatten()) {
        res[i] = c
    }
    res.into_iter().collect()
}

fn fence_levels(num: usize) -> impl Iterator<Item = usize> {
    (0..num - 1).chain((1..num).rev()).cycle()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3),
            "WECRLTEERDSOEEFEAOCAIVDEN"
        );
        assert_eq!(
            decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3),
            "WEAREDISCOVEREDFLEEATONCE"
        );
        assert_eq!(
            encode_rail_fence_cipher("Hello, World!", 3),
            "Hoo!el,Wrdl l"
        );
        assert_eq!(
            decode_rail_fence_cipher("Hoo!el,Wrdl l", 3),
            "Hello, World!"
        );
    }
}
