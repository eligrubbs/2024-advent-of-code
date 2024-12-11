use day_8::alg::{day_8_p1_soln, day_8_p2_soln};

fn main() {
    let unique_antenna_spots: i32 = day_8_p1_soln();
    println!("Unique Antennas: {}", unique_antenna_spots);

    let resonant_antenna_spots: i32 = day_8_p2_soln();
    println!("Unique Antennas: {}", resonant_antenna_spots);
}
