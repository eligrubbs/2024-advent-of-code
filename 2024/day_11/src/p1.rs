use std::{env, fs::read_to_string};


pub fn day_11_p1_soln() -> usize {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let mut line: Vec<Stone> = parse_input(&raw);
    line = blink_x_times(line, 25);
    line.len()
}

pub fn blink_x_times(mut line: Vec<Stone>, times: u32) -> Vec<Stone> {
    for _ in 0..times {
        line = blink_at_line(line);
    }
    line
}


pub fn parse_input(content: &str) -> Vec<Stone> {
    content.split(" ")
           .map(|raw_nums| raw_nums.parse::<u64>().unwrap())
           .map(|num| Stone{num})
           .collect::<Vec<Stone>>()
}

pub fn blink_at_line(stones: Vec<Stone>) -> Vec<Stone> {
    let mut result: Vec<Stone> = vec![];
    stones.iter()
          .map(|st| st.blink())
          .for_each(|s| result.extend_from_slice(&s));

    result
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Stone {
    pub num: u64,
}

impl Stone {
    fn blink(&self) -> Vec<Stone> {
        let dig: u32 = digits(self.num);
        if self.num == 0 {
            vec![Stone{num: 1}]
        } else if dig % 2 == 0 {
            let left_digits: u64 = self.num / (10_u64.pow(dig/2));
            let right_digits: u64 = self.num % (10_u64.pow(dig/2));
            vec![Stone{num: left_digits}, Stone{num: right_digits}]
        } else {
            vec![Stone{num: self.num * 2024}]
        }

    }
}

fn digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blink() {
        // first rule
        let stone1: Stone = Stone{num:0};

        // third rule
        assert_eq!(vec![Stone{num:1}], stone1.blink());
        for i in 1..10 {
            let stone_x: Stone = Stone{num:i};
            assert_eq!(vec![Stone{num:i*2024}], stone_x.blink());
        }
        let stone_odd_big: Stone = Stone{num:123456789};
        assert_eq!(vec![Stone{num:123456789*2024}], stone_odd_big.blink());

        // second rule
        let stone_even_big: Stone = Stone{num:12345678};
        assert_eq!(vec![Stone{num:1234}, Stone{num:5678}], stone_even_big.blink());
    }

    #[test]
    fn test_line_blink() {
        let line1: Vec<Stone> = vec![Stone{num:125}, Stone{num:17}];
        let line2: Vec<Stone> = blink_at_line(line1);
        assert_eq!(vec![Stone{num:253000}, Stone{num:1}, Stone{num:7}], line2);
        assert_eq!(vec![Stone{num:253}, Stone{num:0}, Stone{num:2024}, Stone{num:14168}], blink_at_line(line2));
    }
}