use regex::Regex;

use crate::parser::{parse_direction, ReadDirection::*};


pub fn day_4_p1_soln() -> i32{
    let mut count: i32 = 0;
    count += xmas_count_in_dir(parse_direction(GoNorth));
    count += xmas_count_in_dir(parse_direction(GoNorthEast));
    count += xmas_count_in_dir(parse_direction(GoEast));
    count += xmas_count_in_dir(parse_direction(GoSouthEast));
    count += xmas_count_in_dir(parse_direction(GoSouth));
    count += xmas_count_in_dir(parse_direction(GoSouthWest));
    count += xmas_count_in_dir(parse_direction(GoWest));
    count += xmas_count_in_dir(parse_direction(GoNorthWest));

    count
}

fn xmas_count_in_dir(list: Vec<String>) -> i32{
    list.iter().map(|x| str_xmas_count(x)).sum()
}

fn str_xmas_count(input: &str) -> i32{
    let re = Regex::new(r"XMAS").unwrap();
    re.find_iter(input).map(|_s| 1).collect::<Vec<i32>>().iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xmas_count(){
        let input: String = String::from("XMASXMASXMASMMMMXMAS");
        assert_eq!(4, str_xmas_count(&input));
    }
}