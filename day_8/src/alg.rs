use std::collections::{HashMap, HashSet};

use crate::parser::{Coord, day_8_parse_input};



pub fn day_8_p1_soln() -> i32 {
    let (input, bounds) = day_8_parse_input();
    let mut unique_spots: HashSet<Coord> = HashSet::new();
    for (_antenna_type, antennas) in input.iter() {
        let bob: HashMap<Coord, Vec<(&Coord, &Coord)>> = get_frequency_antinodes(antennas, bounds, true);
        unique_spots.extend(bob.keys());
    }
    unique_spots.len() as i32
}

pub fn day_8_p2_soln() -> i32 {
    let (input, bounds) = day_8_parse_input();
    let mut unique_spots: HashSet<Coord> = HashSet::new();
    for (_antenna_type, antennas) in input.iter() {
        let bob: HashMap<Coord, Vec<(&Coord, &Coord)>> = get_frequency_antinodes(antennas, bounds, false);
        unique_spots.extend(bob.keys());
    }
    unique_spots.len() as i32
}


/// Given a list of antennas, return all of the created antinodes and the lsit of pairs of antennas that created them.
fn get_frequency_antinodes(antennas: &Vec<Coord>, bounds: (i32,i32), is_p1: bool) -> HashMap<Coord, Vec<(&Coord, &Coord)>>{
    let pairs: Vec<(&Coord, &Coord)> = pairs_from_vec(&antennas);
    antinodes_with_parent_pairs(pairs, bounds, is_p1)
}


/// Find all coordinate pairs given a list of coordinates
fn pairs_from_vec(input: &Vec<Coord>) -> Vec<(&Coord, &Coord)> {
    let mut result: Vec<(&Coord,&Coord)> = vec![];
    for left in 0..(input.len()-1) {
        for right in left+1..input.len() {
            result.push((&input[left], &input[right]));
        }
    }
    result
}


/// Finds all antinodes given a list of pairs of coordinates.  
/// Returns a HashMap with (k: antinode, v: Vec<(pairs of coords that create an antinode here)>))
fn antinodes_with_parent_pairs<'a>(all_pairs: Vec<(&'a Coord, &'a Coord)>, bounds: (i32,i32), is_p1: bool) -> HashMap<Coord, Vec<(&'a Coord, &'a Coord)>> {
    let mut result: HashMap<Coord, Vec<(&'a Coord, &'a Coord)>> = HashMap::new();
    for pair in all_pairs {
        let pairs_antinodes: Vec<Coord> = if is_p1 {
            get_antinodes(pair.0, pair.1, bounds)
        } else {
            get_antinodes_resonant_harmonics_accounted_for(pair.0, pair.1, bounds)
        };
        for antinode in pairs_antinodes {
            result.entry(antinode)
                  .and_modify(|parents| parents.push(pair))
                  .or_insert(vec![pair]);
        }
    }
    result
}

/// Given two coordinates, get the antinodes.
/// 
/// An antinode is a point that is twice as far from one coordinate as the other.
/// 
/// Here is how to calculate an antinode:
/// 1. Find the (dx,dy) between the points
///     - Be mindful of the direction this is calculated in (p1 - p2) for example
/// 2. With direction accounted for, add it to both points in the direction away from the other
///     - This creates 2 antinodes
/// 
/// Just kidding with the below, the problem said this is impossible
/// 3. If BOTH dx, dy are divisible by 3, there are two antinodes inbetween them that line up on the coordinate space
fn get_antinodes(lc: &Coord, rc: &Coord, bounds:(i32,i32)) -> Vec<Coord> {
    let mut results: Vec<Coord> = vec![];

    // (dx,dy) pointing towards lc from rc
    let (dx, dy) = (lc.c - rc.c, lc.r - rc.r);

    let outside_closer_to_l: Coord = Coord{r: lc.r + dy, c: lc.c + dx};
    if coord_in_bounds(&outside_closer_to_l, bounds.0, bounds.1) {
        results.push(outside_closer_to_l);
    }
    let outside_closer_to_r: Coord = Coord{r: rc.r - dy, c: rc.c - dx};
    if coord_in_bounds(&outside_closer_to_r, bounds.0, bounds.1) {
        results.push(outside_closer_to_r);
    }

    // if (dx % 3 == 0) && (dy % 3 == 0) { // inside might be possible - but problem said otherwise
    //     let (third_dx, third_dy) = (2 * dx / 3, 2 * dy / 3);

    //     let inside_closer_to_l: Coord = Coord{r: lc.r - third_dx, c: lc.c - third_dy};
    //     let inside_closer_to_r: Coord = Coord{r: rc.r + third_dx, c: rc.c + third_dy};
    //     candidates.push(inside_closer_to_l);
    //     candidates.push(inside_closer_to_r);
    // }

    results
}

fn coord_in_bounds(cd: &Coord, max_row: i32, max_col: i32) -> bool {
    (cd.c >= 0 && cd.c <= max_col) && (cd.r >= 0 && cd.r <= max_row)
}


fn get_antinodes_resonant_harmonics_accounted_for(lc: &Coord, rc: &Coord, bounds:(i32,i32)) -> Vec<Coord> {
    let mut results: Vec<Coord> = vec![];

    // pointing towards lc from rc
    let (dx, dy) = (lc.c - rc.c, lc.r - rc.r);

    // go backwards from lc
    let mut curr_coord: Coord = lc.clone();
    while coord_in_bounds(&curr_coord, bounds.0, bounds.1) {
        results.push(curr_coord);
        // update with dy/dx
        curr_coord = Coord{r: curr_coord.r + dy, c: curr_coord.c + dx};
    }

    // go forwards from lc until out of bounds
    curr_coord = rc.clone();
    while coord_in_bounds(&curr_coord, bounds.0, bounds.1) {
        results.push(curr_coord);
        curr_coord = Coord{r: curr_coord.r - dy, c: curr_coord.c - dx};
    }

    results
}