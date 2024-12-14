use std::{collections::HashMap, env, fs::read_to_string};
use crate::p1::{Stone, parse_input, blink_x_times, blink_at_line};

pub fn day_11_p2_soln() -> u64 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let line: Vec<Stone> = parse_input(&raw);
    recursive_blink(line, 75)
}

fn recursive_blink(line: Vec<Stone>, times: u32) -> u64 {
    let mut count: u64 = 0;
    let mut shared_info: HashMap<(Stone, u32), u64> = HashMap::new();
    for stone in line {
        count += rec_blink_help(stone, times, &mut shared_info);
    }
    count
}


#[allow(unused_assignments)]
fn rec_blink_help(st: Stone, times: u32, shared_info: &mut HashMap<(Stone,u32),u64>) -> u64 {
    if let Some(&seen) = shared_info.get(&(st, times)) {
        seen
    } else {
        let mut result: u64 = 0;
        if times <= 6 { // base -- prune quicker
            result = blink_x_times(vec![st], times).len() as u64;
        } else { // gotta blink
            let line: Vec<Stone> = blink_at_line(vec![st]);
            let mut child_count: u64 = 0;
            for stone in line {
                child_count += rec_blink_help(stone, times-1, shared_info);
            }
            result = child_count
        }
        shared_info.insert((st, times), result);
        result
    }
}


#[cfg(test)] 
mod test {
    use super::*;

    #[test]
    fn test_recursive_blink_simple() {
        // first rule
        let stone1: Stone = Stone{num:0};

        // third rule
        assert_eq!(1, recursive_blink(vec![stone1], 1));
        for i in 1..10 {
            let stone_x: Stone = Stone{num:i};
            assert_eq!(1, recursive_blink(vec![stone_x], 1));
        }
        let stone_odd_big: Stone = Stone{num:123456789};
        assert_eq!(1, recursive_blink(vec![stone_odd_big], 1));

        // second rule
        let stone_even_big: Stone = Stone{num:12345678};
        assert_eq!(2, recursive_blink(vec![stone_even_big], 1));
    }

    #[test]
    fn test_recursive_blink_spec() {
        let line1: Vec<Stone> = vec![Stone{num:125}, Stone{num:17}];
        assert_eq!(3, recursive_blink(line1.clone(), 1));
        assert_eq!(4, recursive_blink(line1.clone(), 2));
        assert_eq!(22, recursive_blink(line1.clone(), 6));
        assert_eq!(55312, recursive_blink(line1.clone(), 25));
    }
}