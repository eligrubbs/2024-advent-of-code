use regex::Regex;
use strum::IntoEnumIterator;

use crate::parser::{parse_direction, parse_input, ReadDirection};


pub fn day_4_p1_soln() -> i32{
    let mut count: i32 = 0;
    for dir in ReadDirection::iter(){
        count += xmas_count_in_dir(parse_direction(dir));
    }

    count
}

fn xmas_count_in_dir(list: Vec<String>) -> i32{
    list.iter().map(|x| str_xmas_count(x)).sum()
}

fn str_xmas_count(input: &str) -> i32{
    let re = Regex::new(r"XMAS").unwrap();
    re.find_iter(input).map(|_s| 1).collect::<Vec<i32>>().iter().sum()
}


pub fn day_4_p2_soln() -> i32{
    let base: Vec<String> = parse_input().lines().map(|x| x.to_string()).collect();
    let (length, width) = (base.len(), base.get(0).unwrap().len());
    let mut count: i32 = 0;
    let ok_chars: [char;2] = ['M','S'];

    for row in 1..(length-1) {
        for col in 1..(width-1) {
            let char_at_pos: char = base.get(row).unwrap().chars().nth(col).unwrap();
            if char_at_pos == 'A' {
                let tl = base.get(row-1).unwrap().chars().nth(col-1).unwrap();
                let tr = base.get(row-1).unwrap().chars().nth(col+1).unwrap();
                let bl = base.get(row+1).unwrap().chars().nth(col-1).unwrap();
                let br = base.get(row+1).unwrap().chars().nth(col+1).unwrap();
                if ( ok_chars.contains(&tl) && ok_chars.contains(&tr) && ok_chars.contains(&bl) && ok_chars.contains(&br) )
                && (tl != br && tr != bl) {
                    count += 1;
                }
            }
        }
    }
    count
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