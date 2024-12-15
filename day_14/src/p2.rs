// If they are a christmas tree, a bunch of them are stacked together.
// The tree is likely the only shape where they are all bunched up like that, spreading from there into randomness
// Therefore I look for the minimum after 1000 seconds and then print at what second the first minimum occured

use std::{env, fs::read_to_string, i32};
use crate::p1::{Robot, parse_input, get_saftey_factor, move_all_for_x_secs};

pub fn day_14_p2_soln() -> u32 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let mut bots: Vec<Robot> = parse_input(&raw, 101, 103);

    track_saftey(&mut bots, 10000)
}

pub fn track_saftey(bots: &mut Vec<Robot>, max_secs: u32) -> u32 {
    let mut sec_of_min: u32 = 0;
    let mut factor_of_min: i32 = i32::MAX;

    for sec in 0..max_secs {
        let contender: i32 = get_saftey_factor(&bots);
        if contender < factor_of_min {
            factor_of_min = contender;
            sec_of_min = sec;
        }
        move_all_for_x_secs(bots, 1);
    }


    sec_of_min
}