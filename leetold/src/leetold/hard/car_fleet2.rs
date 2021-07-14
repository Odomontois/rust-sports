use std::{cmp::Reverse, ops::Range, usize, vec};

pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
    let mut cars: Vec<_> = cars.into_iter().enumerate().map(Car::from).collect();
    cars.sort_by_key(|c| Reverse(c.pos));
    let mut results = vec![-1.0f64; cars.len()];
    let mut current = Vec::<Segment>::new();
    for car in cars {
        while let Some(s) = current.pop() {
            if let Some(crash) = s.collide(car) {
                results[car.idx] = crash;
                current.push(Segment::new(s.car, crash..s.interval.end));
                current.push(Segment::new(car, 0.0..crash));
                break;
            }
        }
        if current.is_empty() {
            current.push(Segment::new(car, 0.0..std::f64::INFINITY))
        }
    }

    results
}

#[derive(Debug, Clone, Copy)]
struct Car {
    pos: i32,
    speed: i32,
    idx: usize,
}

impl From<(usize, Vec<i32>)> for Car {
    fn from((idx, v): (usize, Vec<i32>)) -> Self {
        Car {
            pos: v[0],
            speed: v[1],
            idx,
        }
    }
}

#[derive(Debug, Clone)]
struct Segment {
    car: Car,
    interval: Range<f64>,
}

impl Segment {
    fn new(car: Car, interval: Range<f64>) -> Self {
        Self { car, interval }
    }
    fn collide(&self, car: Car) -> Option<f64> {
        if self.car.speed == car.speed {
            return if self.car.pos == car.pos { Some(0.) } else { None };
        }
        let t = (car.pos - self.car.pos) as f64 / (self.car.speed - car.speed) as f64;
        Some(t).filter(|t| self.interval.contains(t))
    }
}

#[test]
fn test() {
    fn check(xs: &[[i32; 2]], exp: &[f64]) {
        assert_eq!(get_collision_times(xs.iter().map(|v| v.to_vec()).collect()), exp)
    }
    check(
        &[[1, 2], [2, 1], [4, 3], [7, 2]],
        &[1.00000, -1.00000, 3.00000, -1.00000],
    );
    check(
        &[[3, 4], [5, 4], [6, 3], [9, 1]],
        &[2.00000, 1.00000, 1.50000, -1.00000],
    );
}
