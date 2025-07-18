use std::env;
use std::fs::read_to_string;

pub fn parse_day_7_input() -> Vec<(i64, Vec<i64>)>{
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    parse_input(&raw)
}


pub fn parse_input(content: &str) -> Vec<(i64, Vec<i64>)>{
    content.lines().into_iter()
                   .map(|line| line.split(": ").collect::<Vec<&str>>())
                   .map(|bob| (bob[0], bob[1].split_whitespace().collect::<Vec<&str>>()))
                   .map(|(total, vals)| (total.parse().unwrap(),
                                                          vals.into_iter()
                                                              .map(|elem| elem.parse()
                                                                                    .unwrap())
                                                              .collect()))
                   .collect()
}

