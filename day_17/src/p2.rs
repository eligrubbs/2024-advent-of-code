use std::{env, fs::read_to_string};
use crate::computer::Computer;
use crate::p1::{parse_input, input_from_str};


pub fn day_17_p2_soln() -> u32 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let (_, orig) = parse_input(&raw);
    let input: Vec<u8> = input_from_str(&orig);


    p2_helper(input, orig)
}

fn p2_helper(input: Vec<u8>, orig: String) -> u32 {
    let mut curr_a: u32 = 0;

    loop {
        let mut comp: Computer = Computer::new(curr_a, 0, 0, input.clone());
        comp.run_until_not_match(&orig);
    
        if comp.get_output() == orig {
            println!("{} {}", comp.get_output(), orig);
            break;
        }
        if curr_a % 1_000_000 == 0 {
            print!(".");
        }
        curr_a += 1;
    }
    println!();
    curr_a
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spec_p2() {
        let input: Vec<u8> = vec![0,1,5,4,3,0];
        let orig: String = "0,1,5,4,3,0".to_string();

        assert_eq!(117440, p2_helper(input, orig));
    }
}