use std::time::{Instant, Duration};
use day_18::grid::Coord;
use day_18::p1::day_18_p1_soln;
use day_18::p2::day_18_p2_soln;

fn main() {
    let now: Instant = Instant::now();

    let steps: usize = day_18_p1_soln();
    println!("Steps: {}", steps);

    let coord: Coord = day_18_p2_soln();
    println!("Coord: {},{}", coord.c, coord.r);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
