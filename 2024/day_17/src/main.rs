use std::time::{Instant, Duration};
use day_17::p1::day_17_p1_soln;
use day_17::p2::day_17_p2_soln;

fn main() {
    let now: Instant = Instant::now();

    let output: String=day_17_p1_soln();
    println!("P1: {}", output);

    let min_a: u64 = day_17_p2_soln();
    println!("P2: {}", min_a);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
