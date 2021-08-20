pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
    let mut obst: Vec<i32> = Vec::with_capacity(obstacles.len());
    let step = |x| {
        let i = obst.binary_search(&(2 * x + 1)).unwrap_err();
        if i == obst.len() {
            obst.push(2 * x)
        } else {
            obst[i] = 2 * x
        };
        i as i32 + 1
    };
    obstacles.into_iter().map(step).collect()
}
