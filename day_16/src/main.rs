use day_16::p1::{day_16_p1_soln, ValueMap};
use day_16::p2::day_16_p2_soln;

fn main() {
    let (v_map, start, min_cost) = day_16_p1_soln();
    println!("Min: {}", min_cost);

    let num: i64 = day_16_p2_soln(&v_map, start, 1);
    print!("Num:{}", num);
}
