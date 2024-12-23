use std::time::{Instant, Duration};
use day_20::p1::day_20_p1_soln;
use day_20::p2::day_20_p2_soln;

fn main() {
    let now: Instant = Instant::now();

    // day_20_p1_soln();
    let savings: u32 = day_20_p1_soln();
    println!("Num >=100 Savings: {}", savings);

    let super_cheats: usize = day_20_p2_soln();
    println!("Super Cheats: {}", super_cheats);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
