use std::env;
use std::fs::read_to_string;
use crate::p1::{parse_input, check_all_goals};

pub fn day_19_p2_soln() -> u64 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let (pats, goals) = parse_input(&raw);

    let results: Vec<u64> = check_all_goals(&goals, &pats);

    results.iter().sum()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spec() {
        let input: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let (pats, goals) = parse_input(input);

        assert_eq!(check_all_goals(&goals, &pats), vec![2,1,4,6,0,1,2,0]);
    }
}