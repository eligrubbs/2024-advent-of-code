use day_14::{p1::day_14_p1_soln, p2::day_14_p2_soln};

fn main() {
    let saftey_factor: i32 = day_14_p1_soln();
    println!("P1 Factor: {}", saftey_factor);

    let tree_seconds: u32 = day_14_p2_soln();
    println!("Secs till Tree: {}", tree_seconds);
}
