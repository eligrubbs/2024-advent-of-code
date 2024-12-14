use day_11::{p1::day_11_p1_soln, p2::day_11_p2_soln};
use std::time::{Instant, Duration};

fn main() {
    let now: Instant = Instant::now();

    let num: usize = day_11_p1_soln();
    println!("25 blink num Stones: {}", num);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let num2: u64 = day_11_p2_soln();
    println!("75 blink num Stones: {}", num2);

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
