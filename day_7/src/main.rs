use day_7::alg::{day_7_p1_soln, day_7_p2_soln};
fn main(){
    let calib_score: i64 = day_7_p1_soln();
    println!("Calib score: {}", calib_score);

    let calib_score_3_operations: i64 = day_7_p2_soln();
    println!("Calib score 3 Operations: {}", calib_score_3_operations);
}
