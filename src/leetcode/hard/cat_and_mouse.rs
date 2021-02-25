use std::collections::HashMap;
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
enum Player {
    Cat,
    Mouse,
}

impl Player {
    fn other(self) -> Self {
        match self {
            Cat => Mouse,
            Mouse => Cat,
        }
    }
}
#[derive(Hash, Eq, Ord, PartialEq, PartialOrd, Debug, Clone, Copy)]
struct Pos {
    cat: usize,
    mouse: usize,
    player: Player,
}

impl Pos {
    fn player_pos(&self) -> usize {
        match self.player {
            Player::Cat => self.cat,
            Player::Mouse => self.mouse,
        }
    }
    fn player_pos_mut(&mut self) -> &mut usize {
        match self.player {
            Player::Cat => &mut self.cat,
            Player::Mouse => &mut self.mouse,
        }
    }

    fn other_pos(&self) -> usize {
        match self.player {
            Player::Cat => self.mouse,
            Player::Mouse => self.cat,
        }
    }

    fn other_pos_mut(&mut self) -> &mut usize {
        match self.player {
            Player::Cat => &mut self.mouse,
            Player::Mouse => &mut self.cat,
        }
    }

    fn preceding<'a>(&'a self, inv: &'a [&'a [usize]]) -> impl Iterator<Item = Pos> + 'a {
        inv[self.other_pos()]
            .iter()
            .map(move |&j| {
                let mut pos = self.clone();
                *pos.other_pos_mut() = j;
                pos.player = pos.player.other();
                pos
            })
            .filter(|p| p.cat != 0)
    }

    fn possible(&self, graph: &[Vec<i32>]) -> usize {
        let next = &graph[self.player_pos()];
        match self.player {
            Mouse => next.len(),
            Cat => next.iter().filter(|&&x| x != 0).count(),
        }
    }
}

const WIN: bool = true;
const LOOSE: bool = false;
use Player::*;

pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
    let mut inversed = vec![vec![]; graph.len()];
    for (i, v) in graph.iter().enumerate() {
        for j in v {
            inversed[*j as usize].push(i)
        }
    }
    let inversed: &[&[usize]] = &inversed.iter().map(|v| v.as_slice()).collect::<Vec<_>>();
    let mut game: HashMap<Pos, Result<bool, usize>> = HashMap::new();
    let situation = |out, cat, mouse, player| (out, Pos { cat, mouse, player });
    let mut stack: Vec<(bool, Pos)> = (1..graph.len())
        .flat_map(|i| {
            vec![
                situation(WIN, i, i, Cat),
                situation(LOOSE, i, i, Mouse),
                situation(LOOSE, i, 0, Cat),
            ]
        })
        .collect();
    for &(w, p) in &stack {
        game.insert(p, Ok(w));
    }

    while let Some((w, pos)) = stack.pop() {
        for p in pos.preceding(inversed) {
            if let Some(Ok(_)) = game.get(&p) {
                continue;
            }
            if w == LOOSE {
                stack.push((WIN, p));
                game.insert(p, Ok(WIN));
                continue;
            }
            let next = || Err(p.possible(&graph));
            let game_res = game.entry(p).or_insert_with(next);
            match game_res {
                Err(c) => {
                    *c -= 1;
                    if *c == 0 {
                        game.insert(p, Ok(LOOSE));
                        stack.push((LOOSE, p));
                    }
                }
                Ok(_) => {}
            }
        }
    }
    let start = Pos {
        mouse: 1,
        cat: 2,
        player: Mouse,
    };

    match game.get(&start).unwrap_or(&Err(0)) {
        Ok(WIN) => 1,
        Ok(LOOSE) => 2,
        Err(_) => 0,
    }
}

#[test]
fn test_cnm() {
    fn check(xs: &[&[i32]], exp: i32) {
        assert_eq!(cat_mouse_game(xs.iter().map(|v| v.to_vec()).collect()), exp)
    }
    // check(&[&[2, 5], &[3], &[0, 4, 5], &[1, 4, 5], &[2, 3], &[0, 2, 3]], 0)
    check(&[&[2, 3], &[3, 4], &[0, 4], &[0, 1], &[1, 2]], 1)
}
