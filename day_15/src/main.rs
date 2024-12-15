use std::time::{Instant, Duration};
use day_15::p1::day_15_p1_soln;
use day_15::p2::day_15_p2_soln;

fn main() {
    let now: Instant = Instant::now();

    let gps: u64 = day_15_p1_soln();
    println!("GPS score: {}", gps);
    let gps_double: u64 = day_15_p2_soln();
    println!("Double Sized GPS score: {}", gps_double);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
