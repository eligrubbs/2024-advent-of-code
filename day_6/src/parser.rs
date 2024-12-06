use std::env;
use std::fs::read_to_string;


pub fn parse_day_6_input() -> (Vec<Vec<bool>>, (usize,usize)){
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    (parse_input(&raw), get_init_guard_pos(&raw).unwrap())
}

fn parse_input(content: &str) -> Vec<Vec<bool>>{
    content.lines()
           .map(|line| line.chars()
                                 .map(|chr| if chr == '#' {true} else {false})
                                 .collect::<Vec<bool>>()
            )
    .collect::<Vec<Vec<bool>>>()
}

fn get_init_guard_pos(content: &str) -> Option<(usize, usize)> {
    let lines: Vec<&str> = content.lines().collect();
    for row in 0..lines.len() {
        let bob = lines[row];
        for col in 0..bob.len() {
            if bob.chars().nth(col).unwrap() == '^' {
                return Some((row, col))
            }
        }
    }
    None // Should Fail
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_init_guard_pos() {
        let raw: &str = "...##.\n##.#^#\n......";
        assert_eq!((1,4), get_init_guard_pos(raw).unwrap());
    }

    #[test]
    fn test_parse_input() {
        let raw: &str = "...##.\n##.#^#\n......";
        let answer: Vec<Vec<bool>> = vec![
            vec![false,false,false,true,true,false],
            vec![true,true,false,true,false,true],
            vec![false,false,false,false,false,false]
        ];
        assert_eq!(answer, parse_input(raw));
    }
}