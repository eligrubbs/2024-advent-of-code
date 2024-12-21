use std::time::{Instant, Duration};
use day_16::p1::day_16_p1_soln;
use day_16::p2::day_16_p2_soln;

fn main() {
    let now: Instant = Instant::now();

    let (v_map, start, min_cost) = day_16_p1_soln();
    println!("Min: {}", min_cost);

    let num: i64 = day_16_p2_soln(&v_map, start, 1);
    println!("Num: {}", num);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
