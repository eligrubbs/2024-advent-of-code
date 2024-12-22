use std::time::{Instant, Duration};
use day_18::p1::day_18_p1_soln;

fn main() {
    let now: Instant = Instant::now();

    let steps: usize = day_18_p1_soln();
    println!("Steps: {}", steps);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
