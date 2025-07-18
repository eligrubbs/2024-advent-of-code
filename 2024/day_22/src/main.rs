use std::time::{Instant, Duration};
use day_22::p1::day_22_p1_soln;
use day_22::p2::day_22_p2_soln;


fn main() {
    let now: Instant = Instant::now();

    let prices: u64 = day_22_p1_soln();
    println!("p1: {}", prices);

    let most_bananas: u64 = day_22_p2_soln();
    println!("p2: {}", most_bananas);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
