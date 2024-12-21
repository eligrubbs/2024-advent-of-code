use std::{env, fs::read_to_string};
use crate::computer::Computer;


pub fn day_17_p1_soln() -> String {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let (mut comp, _) = parse_input(&raw);
    comp.run_to_completion();
    comp.get_output()
}


pub fn parse_input(content: &str) -> (Computer, String) {
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect::<Vec<String>>();
    assert_eq!(lines.len(), 5);

    let reg_a: u64 = lines[0][12..lines[0].len()].parse::<u64>().unwrap();
    let reg_b: u64 = lines[1][12..lines[1].len()].parse::<u64>().unwrap();
    let reg_c: u64 = lines[2][12..lines[2].len()].parse::<u64>().unwrap();

    let input: Vec<u8> = input_from_str(&lines[4][9..lines[4].len()].to_string());

    (Computer::new(reg_a, reg_b, reg_c, input), lines[4][9..lines[4].len()].to_string())
}

/// assumes `s` is a string with only numbers and commas
pub fn input_from_str(s: &str) -> Vec<u8> {
    s.chars().filter(|c| c.is_digit(8)).map(|c| c.to_digit(8).unwrap() as u8).collect()
}