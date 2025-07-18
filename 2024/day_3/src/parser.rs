/*
This file contains code to parse the input and return the two lists.

Input is found at: https://adventofcode.com/2024/day/2/input
*/
use std::env;
use std::fs::read_to_string;
use regex::Regex;

pub fn read_input_file(p1: bool) -> Vec<(i32,i32)> {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    if p1 {
        parse_day_3_lines_p1(path.to_str().unwrap())
    } else {
        parse_day_3_lines_p2(path.to_str().unwrap())
    } 
}

pub fn parse_day_3_lines_p1(filename: &str) -> Vec<(i32,i32)> {
    let file_str: String = read_to_string(filename).unwrap();

    get_groups_from_str(file_str)
}

pub fn parse_day_3_lines_p2(filename: &str) -> Vec<(i32,i32)> {
    let file_str: String = read_to_string(filename).unwrap();

    get_do_groups_from_str(file_str)
}


fn get_groups_from_str(str: String) -> Vec<(i32, i32)> {
    let re: Regex = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)").unwrap();

    re.captures_iter(&str)
      .map(|c| c.extract())
      .map(|(_, [left, right])| (String::from(left).parse().unwrap(),
                                             String::from(right).parse().unwrap())
    ).collect()
}


fn get_do_groups_from_str(str: String) -> Vec<(i32, i32)> {
    let re: Regex = Regex::new(r"do\(\)").unwrap();
    let re_n: Regex = Regex::new(r"don\'t\(\)").unwrap();
    let pairs: Vec<(i32, i32)> = re.split(&str).map(|elem| {
            let bob = re_n.split(elem).collect::<Vec<&str>>();
            *(bob.get(0).unwrap())
        })
        .map(|x| get_groups_from_str(String::from(x))).flatten().collect();
    pairs
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Write;
    use tempdir::TempDir;
    use super::*;

    #[test]
    fn reads_lines_works(){
        let tmp_dir: TempDir = TempDir::new("example").unwrap();
        let file_path: PathBuf  = tmp_dir.path().join("my-temporary-note.txt");
        let mut tmp_file: File = File::create(&file_path).unwrap();

        writeln!(tmp_file, "hellomul(12,32)bobmul(0,)").unwrap();
        writeln!(tmp_file, "1m1ulmul((\\(mul(8,7)").unwrap();

        let result: Vec<(i32, i32)> = parse_day_3_lines_p1(file_path.to_str().unwrap());
        assert_eq!(result, vec![(12,32),(8,7)]);

    }

    #[test]
    fn read_input_works() {
        let inputs: Vec<(i32, i32)> = read_input_file(true);
        assert!(inputs.len() > 0)
    }

    #[test]
    fn test_get_groups(){
        let raw = String::from("mul(10,43)");
        let result = get_groups_from_str(raw);
        assert_eq!(result, vec![(10,43)]);
    }
}