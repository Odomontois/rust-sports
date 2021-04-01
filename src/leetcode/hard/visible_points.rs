use std::f64::consts::PI;

pub fn visible_points(points: Vec<Vec<i32>>, view: i32, location: Vec<i32>) -> i32 {
    let view = view as f64 * PI / 180.0 + 0.0001;
    let angle = |p: Vec<_>| ((location[0] - p[0]) as f64).atan2((location[1] - p[1]) as f64);
    let at_location = points.iter().filter(|&p| p == &location).count() as i32;
    let mut angles: Vec<_> = points.into_iter().filter(|x| x != &location).map(angle).collect();
    let cmp_f64 = |x: &f64, y: &f64| x.partial_cmp(y).unwrap();
    angles.sort_by(cmp_f64);
    angles.extend(angles.clone().into_iter().map(|f| f + 2. * PI));
    let index_of = |f: &f64| angles.binary_search_by(|x| cmp_f64(x, &f)).unwrap_or_else(|e| e);
    let visible = |(i, &f)| i - index_of(&(f - view)) + 1;
    angles.iter().enumerate().map(visible).max().unwrap_or(0) as i32 + at_location
}
