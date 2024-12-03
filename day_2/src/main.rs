use day_2::alg::{day_2_p1_soln, day_2_p2_soln};

fn main() {
    let safe_entries: i32 = day_2_p1_soln();
    println!("safe entries: {}", safe_entries);

    let damp_safe_entries = day_2_p2_soln();
    println!("damp safe entries: {}", damp_safe_entries);
}
