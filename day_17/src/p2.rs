use std::{env, fs::read_to_string};
use crate::computer::Computer;
use crate::p1::{parse_input, input_from_str};


pub fn day_17_p2_soln() -> u64 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let (_, orig) = parse_input(&raw);
    let input: Vec<u8> = input_from_str(&orig);

    backward(input)
}

fn calc_out_from_reg_a(reg_a: u64) -> u64 {
    ((((reg_a % 8) ^ 6) ^ (reg_a / 2_u64.pow(((reg_a % 8) ^ 6) as u32))) ^ 4) % 8
}

fn backward(input: Vec<u8>) -> u64{
    let curr_a: u64 = 0;
    
    let mut dudes_that_match: Vec<u64> = vec![curr_a];
    for elem in input.iter().rev() {
        for old_curr_a in dudes_that_match.clone() { // look at all
            dudes_that_match.remove(0); // remove this element
            for potential_a in old_curr_a*8..((old_curr_a*8)+8) {
                if calc_out_from_reg_a(potential_a) == (*elem as u64) {
                    dudes_that_match.push(potential_a);
                }
            }
        }
    }
    dudes_that_match.iter().min().unwrap().clone()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_out_from_reg_a() {
        assert_eq!(calc_out_from_reg_a(66171486), 2)
    }
}
