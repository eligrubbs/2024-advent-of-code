use day_6::alg::{day_6_p1_soln, day_6_p2_soln};

fn main() {
    let path_len: usize = day_6_p1_soln();
    println!("Path Len: {}", path_len);

    let square_count: usize = day_6_p2_soln();
    println!("Squares possible {:?}", square_count)
}
