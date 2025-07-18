/*
This file contains code to parse the input and return the two lists.

Input is found at: https://adventofcode.com/2024/day/1/input
*/
use std::env;
use std::fs::read_to_string;

pub fn read_input_file() -> (Vec<i32>, Vec<i32>) {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    parse_day_1_lines(path.to_str().unwrap())
}

pub fn parse_day_1_lines(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let raw_line = line.to_string();
        let mut items: Vec<String> = raw_line.split_ascii_whitespace()
                                         .filter(|&x| ! x.trim().is_empty())
                                         .map(|x| String::from(x)).collect();
        assert!(items.len() == 2);
        list_two.push(items.pop().unwrap().parse::<i32>().unwrap());
        list_one.push(items.pop().unwrap().parse::<i32>().unwrap());
    }


    (list_one, list_two)
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
        writeln!(tmp_file, "12   43").unwrap();
        writeln!(tmp_file, "1   3").unwrap();

        let (list_one, list_two) = parse_day_1_lines(file_path.to_str().unwrap());
        assert_eq!(vec![12,1], list_one);
        assert_eq!(vec![43,3], list_two);
    }

    #[test]
    fn read_input_works() {
        let (list_one, list_two) = read_input_file();

        assert!(list_one.len() == 1000);
        assert!(list_two.len() == 1000);
    }
}