use std::cmp::{max, min};
use std::env;
use std::fs::read_to_string;
use std::collections::HashMap;

use crate::grid::{Coord, Grid};
use crate::p1::{parse_input, update_dists};

pub fn day_20_p2_soln() -> usize {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let mut grid: Grid = parse_input(&raw);
    update_dists(&mut grid);

    let cheats: HashMap<(usize, usize), u32> = super_cheats(&grid);

    cheats_over_100(&cheats)
}

pub fn cheats_over_100(cheats: &HashMap<(usize,usize), u32>) -> usize {
    cheats.iter().filter(|(_, &max_saving)| max_saving >= 100).count()
}

pub fn super_cheats(grid: &Grid) -> HashMap<(usize,usize), u32> {
    let mut results: HashMap<(usize,usize), u32> = HashMap::new();

    let coords: Vec<&Coord> = grid.map.keys().collect();

    for start_ind in 0.. coords.len() {
        let start_coord: &Coord = coords[start_ind];
        for end_ind in start_ind+1..coords.len() {
            let end_coord: &Coord = coords[end_ind];
            let man_dist: u32 = start_coord.manhat_dist(end_coord) as u32;
            if man_dist <= 20 {
                let savings: u32 = grid.map.get(start_coord).unwrap()
                                    .abs_diff(*grid.map.get(end_coord).unwrap()) - man_dist;
                let entry: (usize, usize) = (min(start_ind, end_ind), max(start_ind, end_ind));

                results.entry(entry)
                       .and_modify(|e| {*e = max(*e, savings)})
                       .or_insert(savings);
            }
        }
    }

    results
}