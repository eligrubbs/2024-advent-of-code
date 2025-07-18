/*
This file contains code to parse the input and return the two lists.

Input is found at: https://adventofcode.com/2024/day/2/input
*/
use std::env;
use std::fs::read_to_string;

pub fn read_input_file() -> Vec<Vec<i32>> {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    parse_day_2_lines(path.to_str().unwrap())
}

pub fn parse_day_2_lines(filename: &str) -> Vec<Vec<i32>> {
    let mut inputs: Vec<Vec<i32>> = vec![];

    for line in read_to_string(filename).unwrap().lines() {
        let raw_line: String = line.to_string();
        let items: Vec<i32> = raw_line.split_ascii_whitespace()
                                         .filter(|&x| ! x.trim().is_empty())
                                         .map(|x| String::from(x))
                                         .map(|x| x.parse::<i32>().unwrap())
                                         .collect();
        assert!(items.len() > 1);
        inputs.push(items);
    }
    assert!(inputs.len() > 0);
    inputs
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
        writeln!(tmp_file, "12 43 12 4 2 2").unwrap();
        writeln!(tmp_file, "1 3 5 7 8").unwrap();

        let inputs: Vec<Vec<i32>> = parse_day_2_lines(file_path.to_str().unwrap());
        assert!(inputs.len() == 2);

        let res_1: Vec<i32> = vec![12,43,12,4,2,2];
        let res_2: Vec<i32> = vec![1,3,5,7,8];
        assert_eq!(inputs.get(0).unwrap(), &res_1);
        assert_eq!(inputs.get(1).unwrap(), &res_2);
    }

    #[test]
    fn read_input_works() {
        let inputs: Vec<Vec<i32>> = read_input_file();

        assert!(inputs.len() == 1000);
    }
}