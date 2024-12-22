use std::env;
use std::fs::read_to_string;
use std::{cmp::min, collections::{HashMap, HashSet}};
use crate::grid::{Coord, Direction, Grid};


pub fn day_20_p1_soln() {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let mut grid: Grid = parse_input(&raw);
    djikstras(&mut grid);

    let path: Vec<(Coord, Direction)> = find_path(&grid);
    println!("{}",path.len());
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


/// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
pub fn djikstras(grid: &mut Grid) {
    // Step 1 & 2
    // all coords have inifinite distance except the start, which is 0.
    let mut unvisited: HashSet<Coord> = grid.map.keys().map(|k| k.clone()).collect();

    while unvisited.len() != 0 && !all_elems_inf(&unvisited, &grid.map) {
        if unvisited.len() % 1000 == 0 {
            println!("{}", unvisited.len());
        }
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
        results.push(min_breadcrumb);
        curr_coord = min_breadcrumb.0;
    }
    results
}

fn all_elems_inf(set: &HashSet<Coord>, map: &HashMap<Coord, u32>) -> bool {
    map.iter().filter(|(c,_)| set.contains(c))
    .all(|(_,&d)| d == u32::MAX)
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
        assert_eq!(path, vec![(Coord::from((3,1)),N)]);
    }
}