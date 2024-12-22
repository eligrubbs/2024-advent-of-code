use std::env;
use std::fs::read_to_string;
use std::collections::HashSet;
use std::cmp::max;

use crate::grid::{Grid, Coord};


pub fn day_18_p1_soln() -> usize {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let mut grid: Grid = grid_from_input(&raw, 71, 71);

    value_iteration(&mut grid);

    let steps: Vec<Coord> = steps_to_end(&grid, 1).unwrap();

    steps.len()-1
}

pub fn parse_input_p1(content: &str) -> HashSet<Coord> {
    content.lines().take(1024)
        .map(|line|{
            let c_val = line.split(",").nth(0).unwrap().parse::<usize>().unwrap();
            let r_val = line.split(",").nth(1).unwrap().parse::<usize>().unwrap();
            Coord::from_usize((r_val, c_val))
        }).collect::<HashSet<Coord>>()
}

pub fn grid_from_input(content: &str, width: usize, height: usize) -> Grid {
    let blocks: HashSet<Coord> = parse_input_p1(content);
    Grid::from_blocks(&blocks, width, height)
}


/// find the optimal value of being in each state
/// returns true/false based on whether it converged or not
pub fn value_iteration(g: &mut Grid) {

    let theta: i64 = 0; 
    let gamma: i64 = 1;
    let min_val: i64 = -500; // prevents infinite loops in unreachable spaces

    'value_loop: loop {

        let mut delta: i64 = 0;

        for row in (0..g.height).rev() { for col in (0..g.width).rev() {
            let curr_coord: Coord = Coord::from_usize((row,col));
            // skip end state and all blocks
            if g.blocks[row][col] || curr_coord == g.end_state { continue; }

            let old_val: i64 = g.value_map[row][col];
            if old_val <= min_val { delta = max(delta, 0); continue; }

            let new_val: i64 = g.max_action_value(&curr_coord, gamma);
            g.value_map[row][col] = new_val;

            delta = max(delta, (old_val - new_val).abs());

        }}

        if delta <= theta { break 'value_loop; }
    }
}

/// Returns None if the end can't be found
pub fn steps_to_end(g: &Grid, gamma: i64) -> Option<Vec<Coord>> {
    let mut curr_pos: Coord = Coord::from((0,0));
    let mut steps: Vec<Coord> = vec![curr_pos];
    let mut prev_pos: Coord = Coord::from((-1,-1));

    let mut iterations = 0;

    while curr_pos != g.end_state && iterations < 1000{
        iterations += 1;
        let (_, next_pos) = g.best_action_and_next_pos_at(&curr_pos, gamma);
        if next_pos == prev_pos {iterations = 1001; break;};
        prev_pos = curr_pos;
        curr_pos = next_pos;
        steps.push(curr_pos);
    }

    if iterations < 1000 { Some(steps) } else { None }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mini() {
        // simple 2x2 grid
        let input: &str = "0,1";
        let mut grid: Grid = grid_from_input(input, 2, 2);

        value_iteration(&mut grid);
        assert_eq!(grid.value_map, vec![vec![-2,-1], vec![0,0]]);
    }

    #[test]
    fn test_spec() {
        let input: &str = "0,3\n1,2\n1,5\n2,4\n3,3\n3,6\n4,2\n4,5\n5,1\n5,4\n6,0\n6,2";
        let mut grid: Grid = grid_from_input(input, 7, 7);

        value_iteration(&mut grid);
        assert_eq!(grid.value_map[grid.width-1][grid.height-2], -1);
    }
}