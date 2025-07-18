use day_5::alg::{day_5_p1_soln,day_5_p2_soln};

fn main() {
    let sum_of_inorder: i32 = day_5_p1_soln();
    println!("In-order middle sum: {}", sum_of_inorder);

    let sum_of_outorder: i32 = day_5_p2_soln();
    println!("Out-order middle snum: {}", sum_of_outorder);
}
