use day_9::p1::day_9_p1_soln;
use day_9::p2::day_9_p2_soln;

fn main() {
    let bob: u64 = day_9_p1_soln();
    println!("Checksum: {}", bob);

    let bob2: u64 = day_9_p2_soln();
    println!("P2 Checksum: {}", bob2);
}
