// (rx, sx) (ry, sy)
// max(rx, sx + ry) max(ry, sy + rx)
pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
    tasks.sort_by(|v0, v1| v0[1].max(v0[0] + v1[1]).cmp(&v1[1].max(v1[0] + v0[1])));
    tasks.into_iter().rev().fold(0, |s, v| (v[0] + s).max(v[1]))
}

#[test]
fn min_effort() {
    fn check(xs: &[[i32; 2]]) {
        println!("{}",
                 minimum_effort(xs.into_iter().map(|v| v.to_vec()).collect()));
    }
    check(&[[1, 2], [2, 4], [4, 8]]);
    check(&[[1, 3], [2, 4], [10, 11], [10, 12], [8, 9]]);
    check(&[[1, 7], [2, 8], [3, 9], [4, 10], [5, 11], [6, 12]]);
}