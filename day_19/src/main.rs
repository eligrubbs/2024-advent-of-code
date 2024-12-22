use std::time::{Instant, Duration};
use day_19::p1::day_19_p1_soln;
use day_19::p2::day_19_p2_soln;

fn main() {
    let now: Instant = Instant::now();

    let num: usize = day_19_p1_soln();
    println!("Total: {}", num);

    let unique: u64 = day_19_p2_soln();
    println!("Unique: {}", unique);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
