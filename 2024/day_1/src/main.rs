
use day_1::alg::{day_1_p1_soln, day_1_p2_soln};

fn main() {
    let dist: i32 = day_1_p1_soln();
    println!("Distance: {}", dist);

    let sim_score = day_1_p2_soln();
    println!("Sim Score: {}", sim_score);
}
