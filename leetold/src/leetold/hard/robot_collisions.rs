use std::cmp::Ordering;

#[derive(Clone, Copy)]
enum Direction {
    L,
    R,
}
#[derive(Clone, Copy)]
struct Robot {
    position: i32,
    health: i32,
    direction: Direction,
    index: usize,
}
pub fn survived_robots_healths(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32> {
    let robots = positions.into_iter().zip(healths).zip(directions.chars()).enumerate();
    let robots = robots.filter_map(|(index, ((position, health), dir))| {
        Some(Robot {
            position,
            health,
            index,
            direction: match dir {
                'L' => Direction::L,
                'R' => Direction::R,
                _ => return None,
            },
        })
    });
    let mut robots: Vec<Robot> = robots.collect();
    robots.sort_by_key(|robot| robot.position);
    let mut leftys = Vec::<Robot>::new();
    let mut survived = Vec::<Robot>::new();
    while let Some(mut robot) = robots.pop() {
        match robot.direction {
            Direction::L => leftys.push(robot),
            Direction::R => {
                if let Some(mut lefty) = leftys.pop() {
                    match robot.health.cmp(&lefty.health) {
                        Ordering::Less => {
                            lefty.health -= 1;
                            leftys.push(lefty)
                        }
                        Ordering::Equal => {}
                        Ordering::Greater => {
                            robot.health -= 1;
                            robots.push(robot);
                        }
                    }
                } else {
                    survived.push(robot)
                }
            }
        }
    }
    survived.extend(leftys);
    survived.sort_by_key(|robot| robot.index);
    survived.iter().map(|robot| robot.health).collect()
}
