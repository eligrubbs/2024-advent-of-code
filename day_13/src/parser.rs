use regex::Regex;

pub fn parse_input(content: &str, is_p2: bool) -> Vec<Vars> {
    let mut result: Vec<Vars> = vec![];

    let mut line_iter = content.lines();
    let a_regex: Regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let b_regex: Regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    while let (Some(a), Some(b), Some(p)) = (line_iter.next(), line_iter.next(), line_iter.next() ) {
        line_iter.next(); // skip new line

        let (a_x, a_y) = nums_from_str(a, &a_regex);
        let (b_x, b_y) = nums_from_str(b, &b_regex);
        let (mut p_x, mut p_y) = nums_from_str(p, &prize_regex);
        if is_p2 { (p_x, p_y) = (p_x+10000000000000, p_y+10000000000000); }
        let m: Vars = Vars::from((a_x,a_y, b_x, b_y, p_x,p_y));
        result.push(m);
    }
    result
}

fn nums_from_str(input: &str, reg: &Regex) -> (i64, i64) {
    let (_, [x_raw, y_raw]) = reg.captures(input).unwrap().extract();
    (x_raw.parse::<i64>().unwrap(), y_raw.parse::<i64>().unwrap())
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vars{ 
    pub a_x: i64,
    pub a_y: i64,
    pub b_x: i64,
    pub b_y: i64,
    pub p_x: i64,
    pub p_y: i64
}

impl Vars {
    pub fn from(vars: (i64, i64, i64, i64, i64, i64)) -> Vars {
        Vars{a_x: vars.0, 
             a_y: vars.1,
             b_x: vars.2,
             b_y: vars.3,
             p_x: vars.4,
             p_y: vars.5}
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        let result: Vec<Vars> = parse_input(input, false);
        let ans: Vec<Vars> = vec![Vars::from((94,34, 22,67, 8400,5400))];

        assert_eq!(result, ans);

        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";
        let result: Vec<Vars> = parse_input(input, false);
        let ans: Vec<Vars> = vec![Vars::from((94,34, 22,67, 8400,5400)),
        Vars::from((26,66, 67,21, 12748,12176))];

        assert_eq!(result, ans);
    }
}