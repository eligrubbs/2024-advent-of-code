use std::env;
use std::fs::read_to_string;
use std::collections::HashMap;

use crate::grid::{Coord, Grid};

pub fn day_20_p1_soln() -> u32 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let mut grid: Grid = parse_input(&raw);
    update_dists(&mut grid);

    glitch_combo_savings(&grid)
}


pub fn parse_input(content: &str) -> Grid {
    let mut map: HashMap<Coord, u32> = HashMap::new();
    let mut typs: HashMap<Coord, char> = HashMap::new();
    let mut start_coord: Coord = Coord::from((0,0));
    let mut end_coord: Coord = Coord::from((0,0));

    for (row, line) in content.lines().enumerate() {
        for (col, chr) in line.char_indices() {
            if chr == '#' { continue; }
            let curr: Coord = Coord::from((row,col));
            map.insert(curr, u32::MAX);
            typs.insert(curr, chr);

            if chr == 'S' {start_coord = curr; map.entry(curr).and_modify(|e|{*e=0;}); }
            if chr == 'E' {end_coord = curr;}

        }
    }

    Grid{map, typs, start: start_coord, end: end_coord}
}


/// There is only 1 path from start to end.
/// 
/// Only start position and end position have 3 wall neighbors
pub fn update_dists(grid: &mut Grid) {
    let mut curr = grid.start;

    while curr != grid.end {
        for (next, _) in grid.get_neighbors(&curr) {
            let next_val: u32 = grid.map.get(&next).unwrap().clone();
            if next_val == u32::MAX {
                let curr_val: u32 = grid.map.get(&curr).unwrap().clone();
                grid.map.entry(next).and_modify(|e|{*e = 1 + curr_val;});
                curr = next;
            }
        }
    }
}


pub fn glitch_combo_savings(grid: &Grid) -> u32 {
    let coords: Vec<&Coord> = grid.map.keys().collect();
    let mut over_100: u32 = 0;
    for start_ind in 0.. coords.len() {
        let start_coord: &Coord = coords[start_ind];
        for end_ind in start_ind+1..coords.len() {
            let end_coord: &Coord = coords[end_ind];
            if start_coord.manhat_dist(end_coord) == 2 {
                let savings: u32 = grid.map.get(start_coord).unwrap()
                .abs_diff(*grid.map.get(end_coord).unwrap()) - 2;
                if savings >= 100 {
                    over_100 += 1;
                }
            }
        }
    }
    over_100
}