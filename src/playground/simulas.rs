use rand::Rng;

fn simulate(v: &mut [u8]) -> usize {
    let mut r = rand::thread_rng();
    v.fill(0);
    for step in 0.. {
        let i = r.gen_range(0..v.len() * 8);
        let c = &mut v[i / 8];
        let m = 1 << (i % 8);
        if *c & m != 0 {
            return step;
        }
        *c |= m;
    }
    panic!("panic!")
}

fn average(vec_size: usize, sample_size: usize) -> f64 {
    let mut v = vec![0; vec_size];
    (0..sample_size).map(|_| simulate(&mut v) as f64).sum::<f64>() / sample_size as f64
}

fn predicted(n: usize) -> f64 {
    (2..=n)
        .scan(1. / n as f64, |q, i| {
            let (n, i) = (n as f64, i as f64);
            let res = *q * i * (i - 1.);
            *q *= n - i + 1.;
            *q /= n;
            Some(res)
        })
        .sum()
}

#[test]
fn test() {
    const N: usize = 800;
    const S: usize = 100;
    let avg = average(N / 8, S);
    let exp = predicted(N);
    println!(" average of {S} tryouts on {N} is {avg}");
    println!(" predicted {N} = {exp}");
}
