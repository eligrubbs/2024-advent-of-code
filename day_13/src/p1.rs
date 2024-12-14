use std::{env, fs::read_to_string};
use crate::parser::{Vars, parse_input};


pub fn day_13_p1_soln() -> i64 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let all_probs: Vec<Vars> = parse_input(&raw, false);
    let mut cost = 0;
    for prob in all_probs {
        cost += solve_one_machine(prob);
    }
    cost
}


pub fn solve_one_machine(vs: Vars) -> i64 {
    let mut cost: i64 = 0;

    let a_numerator: i64 = (vs.p_x * vs.b_y) - (vs.p_y * vs.b_x);
    let b_numerator: i64 = (vs.p_y * vs.a_x) - (vs.p_x * vs.a_y);
    let denom: i64 = vs.b_y*vs.a_x - vs.b_x*vs.a_y;
    let a: i64 = a_numerator / denom;
    let b: i64 = b_numerator / denom;

    if a_numerator % denom == 0 && b_numerator % denom == 0 {
        cost += (3*a) + b
    }

    cost
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spec() {
        let spec_cost: i64 = solve_one_machine(Vars::from((94,34,22,67,8400,5400)));
        assert_eq!(spec_cost, 280);

        // example 2 unsolvable
        let spec_cost: i64 = solve_one_machine(Vars::from((26,66,67,21,12748,12176)));
        assert_eq!(spec_cost, 0);
    }
}