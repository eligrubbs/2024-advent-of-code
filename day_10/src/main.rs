use day_10::p1::day_10_p1_soln;
use day_10::p2::day_10_p2_soln;

fn main() {
    let trail_sum = day_10_p1_soln();
    println!("Trail Sum: {}", trail_sum);

    let trail_rating_sum = day_10_p2_soln();
    println!("Trail Rating Sum: {}", trail_rating_sum);
}
