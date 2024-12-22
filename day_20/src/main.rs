use std::time::{Instant, Duration};
use day_20::p1::day_20_p1_soln;

fn main() {
    let now: Instant = Instant::now();

    day_20_p1_soln();

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
