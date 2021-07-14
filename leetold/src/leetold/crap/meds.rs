#[allow(dead_code)]
pub fn med_chk(xs: &[i32], ys: &[i32]) -> (f64, f64) {
    (med2(xs, ys, true), med_plus(xs, ys))
}
#[allow(dead_code)]
pub fn med2(xs: &[i32], ys: &[i32], debug: bool) -> f64 {
    if debug {println!("med2 {:?} {:?}", xs, ys);}
    if xs.len() < ys.len() {
        med2(ys, xs, debug)
    } else if ys.is_empty() {
        median(xs)
    } else if xs.len() - ys.len() > 2 {
        let u = (xs.len() - ys.len() - 1) / 2;
        med2(&xs[u..xs.len() - u], ys, debug)
    } else if xs.len() <= 5 && ys.len() <= 5 {
        med_plus(xs, ys)
    } else {
        let u = xs.len().min(ys.len()) / 2;
        let v = ys.len() - u;
        let w = xs.len() - u ;
        if xs[u] <= ys[u] {
            med2(&xs[u - 1..], &ys[..v + 1], debug)
        } else {
            med2(&xs[..w + 1], &ys[u - 1..], debug)
        }
    }
}
#[allow(dead_code)]
fn med_plus(xs: &[i32], ys: &[i32]) -> f64 {
    let mut zs = xs.to_vec();
    zs.extend(ys);
    zs.sort();
    median(zs.as_slice())
}
#[allow(dead_code)]
fn median(xs: &[i32]) -> f64 {
    let m = xs.len() / 2;
    if xs.len() % 2 == 0 { (xs[m - 1] + xs[m]) as f64 / 2.0 } else { xs[m] as f64 }
}

#[allow(dead_code)]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    med2(nums1.as_slice(), nums2.as_slice(), false)
}