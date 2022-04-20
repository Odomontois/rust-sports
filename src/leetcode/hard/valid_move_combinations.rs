pub fn count_combinations<A, VA, B, VB>(pieces: VA, positions: VB) -> i32
where
    A: AsRef<str> + Clone,
    VA: AsRef<[A]>,
    B: AsRef<[i32]> + Clone,
    VB: AsRef<[B]>,
{
    let vects = product(
        pieces
            .as_ref()
            .iter()
            .zip(positions.as_ref())
            .map(|(name, vp)| starts(name.as_ref(), vp.as_ref()[0] as i8, vp.as_ref()[1] as i8)),
    );
    vects.map(|start| moves(&start)).sum::<i32>() as i32
}
#[derive(Clone, Copy, Debug)]
struct Piece {
    x: i8,
    y: i8,
    dx: i8,
    dy: i8,
}
impl Piece {
    fn go(mut self) -> Option<Self> {
        self.x += self.dx;
        self.y += self.dy;
        ((1..=8).contains(&self.x) && (1..=8).contains(&self.y)).then(|| self)
    }
    fn stop(mut self) -> Self {
        self.dx = 0;
        self.dy = 0;
        self
    }
}

fn product<'a, A: Clone + 'a, B>(mut xs: impl Iterator<Item = B> + Clone + 'a) -> Box<dyn Iterator<Item = Vec<A>> + 'a>
where
    B: IntoIterator<Item = A> + 'a,
{
    let add = |x: A| {
        move |mut v: Vec<A>| {
            v.push(x.clone());
            v
        }
    };
    let firsts = if let Some(x) = xs.next() {
        x.into_iter()
    } else {
        return Box::new(std::iter::once(vec![]));
    };
    let products = move |x: A| product(xs.clone()).into_iter().map(add(x));
    Box::new(firsts.flat_map(products))
}

fn starts(name: &str, x: i8, y: i8) -> Vec<Piece> {
    const DS: [[i8; 2]; 8] = [[0, 1], [0, -1], [1, 0], [-1, 0], [1, 1], [1, -1], [-1, 1], [-1, -1]];
    let mask = match name {
        "rook" => 0b00001111,
        "bishop" => 0b11110000,
        "queen" => 0b11111111,
        _ => unreachable!(),
    };

    (0..8)
        .filter(|&i| mask & (1 << i) != 0)
        .filter_map(|i| {
            Piece {
                x,
                y,
                dx: DS[i][0],
                dy: DS[i][1],
            }
            .go()
        })
        .chain(Some(Piece { x, y, dx: 0, dy: 0 }))
        .collect()
}

fn nexts(pieces: &[Piece]) -> impl Iterator<Item = Vec<Piece>> + '_ {
    let mut mask = 0;

    for (i, piece) in pieces.iter().enumerate() {
        if piece.dx != 0 || piece.dy != 0 {
            mask |= 1 << i
        }
    }
    let next = |m, i, p: Piece| (m & (1 << i) == 0).then(|| p.stop()).or(p.go());
    (1..=mask)
        .filter(move |m| m & mask != 0 && m & (!mask) == 0)
        .filter_map(move |m| pieces.iter().enumerate().map(|(i, &p)| next(m, i, p)).collect())
}

fn moves(pieces: &[Piece]) -> i32 {
    let mut occupied: u64 = 0;
    for piece in pieces {
        let place = (piece.x - 1) * 8 + piece.y - 1;
        if occupied & (1 << place) != 0 {
            return 0;
        }
        occupied |= 1 << place
    }
    1 + nexts(pieces).map(|ref v| moves(v)).sum::<i32>()
}

#[test]
fn single_rook() {
    assert_eq!(15, count_combinations(["rook"], [[1, 1]]))
}

#[test]
fn single_queen() {
    assert_eq!(22, count_combinations(["queen"], [[1, 1]]))
}

#[test]
fn single_bishop() {
    assert_eq!(12, count_combinations(["bishop"], [[4, 3]]))
}

#[test]
fn two_rooks() {
    assert_eq!(223, count_combinations(["rook", "rook"], [[1, 1], [8, 8]]))
}

#[test]
fn two_rooks_queen_bishop() {
    assert_eq!(
        32176,
        count_combinations(["rook", "rook", "queen", "bishop"], [[1, 1], [1, 8], [8, 1], [8, 8]])
    )
}
