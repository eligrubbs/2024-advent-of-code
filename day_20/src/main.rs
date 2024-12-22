use std::time::{Instant, Duration};

fn main() {
    let now: Instant = Instant::now();

    println!("Hello, world!");

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
