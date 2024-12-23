use std::env;
use std::fs::read_to_string;
use std::{cmp::min, collections::{HashMap, HashSet}};
use strum::IntoEnumIterator;

use crate::grid::{Coord, Direction, Grid};

/*
I did not read the spec correctly.

There is only 1 path, so no path-finding is neccesary.
Only need to know distance from start for each dot

*/

pub fn day_20_old_p1_soln() {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let mut grid: Grid = parse_input(&raw);
    djikstras(&mut grid);

    let path: Vec<(Coord, Direction)> = find_path(&grid);
    println!("{}",path.len());
    let glitches: Vec<(usize, usize, usize)> = all_glitch_options(&grid, &path);

    let num: usize = glitches_at_least_100(&glitches);
    println!("Glitches: {}", num);

}

pub fn parse_input(content: &str) -> Grid {
    let mut map: HashMap<Coord, u32> = HashMap::new();
    let mut typs: HashMap<Coord, char> = HashMap::new();
    let mut start_coord: Coord = Coord::from((0,0));
    let mut end_coord: Coord = Coord::from((0,0));

    for (row, line) in content.lines().enumerate() {
        for (col, chr) in line.char_indices() {
            let curr: Coord = Coord::from((row,col));
            map.insert(curr, u32::MAX);
            typs.insert(curr, chr);

            if chr == 'S' {start_coord = curr; map.entry(curr).and_modify(|e|{*e=0;}); }
            if chr == 'E' {end_coord = curr;}
        }
    }

    // content.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
    Grid{map, typs, start: start_coord, end: end_coord}
}


pub fn glitches_at_least_100(glitches: &Vec<(usize,usize,usize)>) -> usize {
    glitches.iter().map(|(start, end, _)| end-start-2).filter(|&u| u >= 100).count()
}

/// Calculates all of the glitch positions (glitch start, glitch end) and how
/// many nanoseconds (steps) it saves.
/// 
/// Although glitching will save the most if it goes through walls, it need not.  
/// However, to save time, we assume that you would only begin a glitch into a wall,
/// so if there are no walls around a spot, it won't glitch there.
pub fn all_glitch_options(grid: &Grid, path: &Vec<(Coord, Direction)>) -> Vec<(usize,usize,usize)>{

    path.iter().enumerate().map(|(ind,_)| possible_glitches_from(grid, path, ind)).flatten().collect::<Vec<(usize,usize,usize)>>()
}


/// Returns the start, end index pairs (with spot always being start) of glitches that
/// end up back on the path
/// 
/// A glitch is a minimum of 2 steps in any direction but a maximum of 3.
/// 
/// Want to prevent simple u-turn glitches that spit out path positions 1 infront of behind spot
fn possible_glitches_from(grid: &Grid, path: &Vec<(Coord, Direction)>, ind_spot: usize) -> Vec<(usize,usize, usize)> {
    let mut results: Vec<(usize, usize, usize)> = vec![];
    let spot: Coord = path[ind_spot].0.clone();

    '_first_step: for dir in Direction::iter() { // potentially step into wall
        let next_coord: Coord = spot.go(dir);
        if *grid.typs.get(&next_coord).unwrap() == '#' { // if not on wall, don't bother
            '_second_step: for dir_2 in Direction::iter() {
                let next_next_coord: Coord = next_coord.go(dir_2);

                if let Some(g_ind) = path.iter().position(|&c| c.0 == next_next_coord) {
                    // next_next_coord is NOT spot, nor any part of path before spot
                    if g_ind > ind_spot {
                        results.push((ind_spot, g_ind, 2));
                    }
                }
            } // second step
        }
    } // first step
    results
}


/// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
pub fn djikstras(grid: &mut Grid) {
    // Step 1 & 2
    // all coords have inifinite distance except the start, which is 0.
    let mut unvisited: HashSet<Coord> = grid.typs.iter().filter(|(_,&v)| v != '#').map(|(k,_)| k.clone()).collect();

    while unvisited.len() != 0 {
        // Step 3
        let curr_coord: Coord = elem_w_smallest_dist(&unvisited, &grid.map);
        if curr_coord == grid.end { break; }

        // Step 4
        for (neighbor, _) in grid.get_neighbors(&curr_coord) {
            if unvisited.contains(&neighbor) {
                let dist_through_curr: u32 = grid.map.get(&curr_coord).unwrap() + 1;
                grid.map.entry(neighbor)
                         .and_modify(|e|{*e=min(*e, dist_through_curr)});
            }
        }

        // Step 5
        unvisited.remove(&curr_coord);
    }

}

pub fn find_path(grid: &Grid) -> Vec<(Coord, Direction)> {
    let mut results: Vec<(Coord, Direction)> = vec![];
    let mut curr_coord: Coord  = grid.end;

    while curr_coord != grid.start {
        // get direction of neighbor with minimum path
        let min_breadcrumb: (Coord, Direction) = grid.get_neighbors(&curr_coord).iter().min_by(|(coord_a,_),(coord_b,_)| {
            grid.map.get(coord_a).unwrap().cmp(grid.map.get(coord_b).unwrap())
        }).unwrap().clone();
        results.insert(0,(min_breadcrumb.0, min_breadcrumb.1.opp()));
        curr_coord = min_breadcrumb.0;
    }
    results.insert(0,(curr_coord, Direction::N));
    results
}

fn elem_w_smallest_dist(set: &HashSet<Coord>, map: &HashMap<Coord, u32>) -> Coord {
    map.iter().filter(|(c,_)| set.contains(c))
        .min_by(|&a,&b| 
                        a.1.cmp(&b.1)).unwrap().0.clone()
}


#[cfg(test)]
mod test {
    use super::*;
    use Direction::*;

    #[test]
    fn test_path() {
        let input: &str=
"#####
#...#
#.#.#
#S#E#
#####";
        let mut grid: Grid = parse_input(input);
        djikstras(&mut grid);

        let path: Vec<(Coord, Direction)> = find_path(&grid);
        assert_eq!(path, vec![
            (Coord { r: 3, c: 1 }, N), (Coord { r: 2, c: 1 }, N),
            (Coord { r: 1, c: 1 }, E), (Coord { r: 1, c: 2 }, E),
            (Coord { r: 1, c: 3 }, S), (Coord { r: 2, c: 3 }, S), 
            (Coord { r: 3, c: 1 }, N)]);
    }
}