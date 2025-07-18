use std::{env, fs::read_to_string};
use crate::parser::{Vars, parse_input};
use crate::p1::solve_one_machine;


pub fn day_13_p2_soln() -> i64 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let all_probs: Vec<Vars> = parse_input(&raw, true);
    let mut cost = 0;
    for prob in all_probs {
        cost += solve_one_machine(prob);
    }
    cost
}