use std::env;
use std::fs::read_to_string;
use std::collections::{HashSet, HashMap};


pub fn day_19_p1_soln() -> usize {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let (pats, goals) = parse_input(&raw);

    let results: Vec<u32> = check_all_goals(&goals, &pats);

    results.iter().filter(|&&b| b != 0 ).count()
}


pub fn parse_input(content: &str) -> (HashSet<String>, Vec<String>) {
    let mut cont_iter = content.lines();
    let patterns: HashSet<String> = cont_iter.next().unwrap().trim().to_string().split(", ").map(|s|s.to_string()).collect::<HashSet<String>>();
    //skip new line
    cont_iter.next();

    let goals: Vec<String> = cont_iter.map(|line| line.to_string())
            .collect::<Vec<String>>();

    (patterns, goals)
}


pub fn check_all_goals(goals: &Vec<String>, pats: &HashSet<String>) -> Vec<u32> {
    let mut results: Vec<u32> = vec![];
    let mut iters = 0;
    let mut cache: HashMap<String, u32> = HashMap::new();
    for goal in goals {
        iters += 1;
        println!("{}", iters);
        results.push(can_make(goal, pats, &mut cache));
    }
    results
}


pub fn can_make(goal: &String, pats: &HashSet<String>, cache: &mut HashMap<String, u32>) -> u32 {
    if cache.contains_key(goal) {return cache.get(goal).unwrap().clone(); }
    if goal.is_empty() { return 1; }
    let mut total: u32 = 0;
    for pat in pats {
        if goal.starts_with(pat) {
            let new_goal: String = goal.strip_prefix(pat).unwrap().to_string();
            let result: u32 = can_make(&new_goal, pats, cache);
            cache.insert(new_goal, result);
            total += result;
        }
    }
    total
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_make() {
        let pats: HashSet<String> = HashSet::from(["r".to_string()]);
        let mut cache: HashMap<String, u32> = HashMap::new();

        assert!(can_make(&"r".to_string(), &pats, &mut cache) > 0);
        assert!(can_make(&"rr".to_string(), &pats, &mut cache) > 0);

        assert_eq!(can_make(&"rb".to_string(), &pats, &mut cache), 0);
        assert_eq!(can_make(&"br".to_string(), &pats, &mut cache), 0);
        assert_eq!(can_make(&"b".to_string(), &pats, &mut cache), 0);
    }

    #[test]
    fn test_spec() {
        let input: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let (pats, goals) = parse_input(input);

        assert_eq!(check_all_goals(&goals, &pats).iter().map(|n| *n != 0).collect::<Vec<bool>>(), vec![true,true,true,true,false,true,true,false]);
    }
}