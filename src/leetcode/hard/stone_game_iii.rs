pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
    let mut stones = vec![0, 0];
    stones.extend(stone_value.into_iter().rev());
    let res: i32 = stones.windows(3).fold([0, 0, 0], |p, v| {
        [
            p[1],
            p[2],
            (v[2] - p[2]).max(v[2] + v[1] - p[1]).max(v[2] + v[1] + v[0] - p[0]),
        ]
    })[2];
    ["Bob", "Tie", "Alice"][(res.signum() + 1) as usize].to_string()
}
