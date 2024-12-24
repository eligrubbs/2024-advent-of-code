use std::env;
use std::fs::read_to_string;
use crate::keypads::{NumberPad, DirectionalPad};

pub fn day_21_p1_soln() -> u32{
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let codes: Vec<String> = parse_input(&raw);

    get_complexity(&codes)
}

pub fn parse_input(content: &str) -> Vec<String> {
    content.lines().map(|c|c.to_string()).collect()
}

pub fn get_complexity(codes: &Vec<String>) -> u32 {
    let mut complexity = 0;

    for code in codes {
        let val: u32 = code[0..code.len()-1].trim_start_matches("0").parse::<u32>().unwrap();
        let dirs: String = num_code_to_dirs(&code);
        println!("{} {}", val, dirs.len());
        complexity += val * (dirs.len() as u32);
    }
    complexity
}

/// converts a code like 023A into the directions for the super-abstracted human to use.
/// 
///
pub fn num_code_to_dirs(code: &str) -> String {

    let mut numpad: NumberPad = NumberPad::from_char('A');
    let mut radiated_dirpad: DirectionalPad = DirectionalPad::from_char('A');
    let mut freezing_dirpad: DirectionalPad = DirectionalPad::from_char('A');

    let mut result: String = String::new();
    for code_char in code.chars() {
        let numpad_dirs_for_radiated: String = numpad.move_to(code_char);
        for radiated_char in numpad_dirs_for_radiated.chars() {
            let radpad_dirs_for_freezing: String = radiated_dirpad.move_to(radiated_char);
            for freezing_char in radpad_dirs_for_freezing.chars() {
                // print!("I");
                let freezepad_dirs_for_human: String = freezing_dirpad.move_to(freezing_char);
                result.push_str(&freezepad_dirs_for_human);
            }
        }
    }
    result

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_dirs() {
        let code: &str = "029A";

        let mut result: String = String::new();
        let mut numpad: NumberPad = NumberPad::from_char('A');
        for chr in code.chars() {
            let res: String = numpad.move_to(chr);
            result.push_str(&res);
        }

        assert_eq!("<A^A>^^AvvvA", result);

        let mut result2: String = String::new();
        let mut dirpad1: DirectionalPad = DirectionalPad::from_char('A');
        for chr in result.chars() {
            let res: String = dirpad1.move_to(chr);
            result2.push_str(&res);
        }
        assert_eq!("v<<A>>^A<A>AvA<^AA>Av<AAA>^A", result2);
    }

    #[test]
    fn test_number_harder() {
        let code: &str = "379A";

        let final_dir: String = num_code_to_dirs(code);
        assert_eq!(64, final_dir.len());
    }

    #[test]
    fn test_example() {
        let code: &str = "029A";

        let final_dir = num_code_to_dirs(code);

        assert_eq!(final_dir, "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
    }

    #[test]
    fn test_spect() {
        let codes = vec!["029A".to_string(),
        "980A".to_string(), "179A".to_string(),"456A".to_string(), "379A".to_string()];

        assert_eq!(126384, get_complexity(&codes));
    }
}
