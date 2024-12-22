use std::env;
use std::fs::read_to_string;
use crate::grid::{Grid, Coord};
use crate::p1::{grid_from_input, value_iteration, steps_to_end};


pub fn day_18_p2_soln() -> Coord {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let mut grid: Grid = grid_from_input(&raw, 71, 71);

    value_iteration(&mut grid);
    let mut _steps: Option<Vec<Coord>> = steps_to_end(&grid, 1);

    for coord in parse_input_p2(&raw) {
        grid.add_block(&coord);
        value_iteration(&mut grid);

        _steps = steps_to_end(&grid, 1);
        if _steps.is_none() {
            return coord
        }
    }

    Coord::from((-1,-1))
}

pub fn parse_input_p2(content: &str) -> Vec<Coord> {
    content.lines()
        .map(|line|{
            let c_val = line.split(",").nth(0).unwrap().parse::<usize>().unwrap();
            let r_val = line.split(",").nth(1).unwrap().parse::<usize>().unwrap();
            Coord::from_usize((r_val, c_val))
        }).collect::<Vec<Coord>>()
}
