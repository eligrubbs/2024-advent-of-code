use std::env;
use std::fs::read_to_string;


pub fn parse_day_5_input() -> (Vec<(i32,i32)>, Vec<Vec<i32>>) {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    parse_input(&raw)
}


/// Read the raw input text file and return two lists
/// 
/// `content`: raw input of file
/// 
/// ### Returns  
/// 1. A list of the rules
/// 2. The pages to print. Each line is ordered, not neccesarily correct.
fn split_input(content: &str) -> (Vec<String>, Vec<String>) {
    let mut rules: Vec<String> = vec![];
    let mut pages: Vec<String> = vec![];

    content.lines().into_iter().for_each(|x| {
        if x.contains("|") {
            rules.push(x.to_string());
        } else if x.contains(",") {
            pages.push(x.to_string());
        }
    });

    (rules, pages)
}


fn parse_input(content: &str) -> (Vec<(i32,i32)>, Vec<Vec<i32>>) {
    let split_input = split_input(content);
    (parse_rules(&split_input.0), parse_pages(&split_input.1))
}

fn parse_rules(rules_raw: &Vec<String>) -> Vec<(i32, i32)>{
    rules_raw.iter().map(|raw| 
                        raw.split("|").collect::<Vec<&str>>())
                    .map(|split| (split.get(0).unwrap().to_string(), split.get(1).unwrap().to_string()))
                    .map(|(before, after)| (before.parse::<i32>().unwrap(), after.parse::<i32>().unwrap()))
                    .map(|(before, after)| (before,after)).collect()
}

fn parse_pages(pages_raw: &Vec<String>) -> Vec<Vec<i32>> {
    pages_raw.iter().map(|x| x.split(","))
                    .map(|vec| vec.map(|x| x.parse::<i32>().unwrap()).collect())
                    .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_input() {
        let input: &str = "23|43\n99|12\n\n12,42,54";
        let result: (Vec<String>, Vec<String>) = split_input(input);
        assert_eq!(result.0, vec!["23|43","99|12"]);
        assert_eq!(result.1, vec!["12,42,54"]);

    }

    #[test]
    fn test_parse_rules() {
        let input: &str = "23|43\n99|12\n\n12,42,54";
        let result: (Vec<String>, Vec<String>) = split_input(input);
        let rule_result: Vec<(i32,i32)> = parse_rules(&result.0);
        assert_eq!(rule_result, vec![(23,43), (99,12)]);
    }

    #[test]
    fn test_parse_pages() {
        let input: &str = "23|43\n99|12\n\n12,42,54";
        let result: (Vec<String>, Vec<String>) = split_input(input);
        let pages_result: Vec<Vec<i32>> = parse_pages(&result.1);
        assert_eq!(pages_result, vec![vec![12,42,54]]);
    }

}