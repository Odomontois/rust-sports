#[cfg(test)]
use std::{thread, time::Duration};

#[test]
#[ignore]
fn join_threads() {
    let cool_thread = thread::spawn(|| {
        println!("Begin cool.");
        thread::sleep(Duration::from_millis(1000));
        println!("End cool.");
    });

    for i in 0..10 {
        println!("Loop: {}.", i);
        thread::sleep(Duration::from_millis(100));
    }

    cool_thread.join().unwrap();
}
