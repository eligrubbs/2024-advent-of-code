use std::time::{Instant, Duration};
use day_13::{p1::day_13_p1_soln, p2::day_13_p2_soln};

fn main() {
    let now: Instant = Instant::now();

    let p1_cost: i64 = day_13_p1_soln();
    println!("P1 cost: {}", p1_cost);

    let p2_cost: i64 = day_13_p2_soln();
    println!("P2 cost: {}", p2_cost);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
