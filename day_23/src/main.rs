use std::time::{Instant, Duration};
use day_23::p1::day_23_p1_soln;
// use day_23::p2::day_23_p2_soln;


fn main() {
    let now: Instant = Instant::now();

    let num_t_3s: usize = day_23_p1_soln();
    println!("p1: {}", num_t_3s);

    // let something: u64 = day_23_p2_soln();
    // println!("p2: {}", something);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}