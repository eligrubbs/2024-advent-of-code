use std::{collections::HashSet, hash::Hash};

use crate::parser::parse_day_6_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BreadCrumb {
    pos: Coord,
    dir: Direction,
}


pub fn day_6_p1_soln() -> usize {
    let (map, init_pos) = parse_day_6_input();
    let (crumbs, _) = track_path(&map, init_pos);

    crumbs.len()
}

pub fn day_6_p2_soln() -> (usize, usize) {
    let (map, init_pos) = parse_day_6_input();
    let (_, bob) = track_path(&map, init_pos);

    bob
}



/// I assume that the guard has a non-overlapping path that takes it out of the map
fn track_path(map: &Vec<Vec<bool>>, init_pos: (usize,usize)) -> (HashSet<Coord>, (usize, usize)){

    // initialize Guard
    let mut pos: Coord = Coord{row: init_pos.0, col: init_pos.1};
    let mut dir: Direction = Direction::North;

    // Keep track of coords seen
    let mut coords_seen: HashSet<Coord> = HashSet::from([pos]);
    let mut crumbs: HashSet<BreadCrumb> = HashSet::from([BreadCrumb{pos,dir}]);

    // Keep track of corners ran into
    let mut obstacle_places: HashSet<Coord> = HashSet::new();
    let mut hypo_count = 0;

    // Keep track if guard has looped
    let has_looped: bool = false;

    'guard_path: while !has_looped {

        // continue until you hit a block or are out of bounds
        while let Some(new_pos) = maybe_move_direction(&map, &pos, &dir, None) {
            // I could move this direction, but lets see what happens if there was a block here
            if path_leads_to_loop(map, &pos, &turn_right_90_deg(&dir), &crumbs, &new_pos) {
                obstacle_places.insert(new_pos);
                hypo_count += 1;
            }
            // take that step in that direction
            pos = new_pos;
            coords_seen.insert(pos);
            crumbs.insert(BreadCrumb{pos, dir});
            
        }
        // Can't continue
        // if about to go out of bounds, stop tracking path;
        if check_pos_out_of_bounds(map, &pos, &dir) {break 'guard_path;}

        // insert crumb saying you hit wall
        crumbs.insert(BreadCrumb{pos, dir});

        // else hit obstable, rotate
        dir = turn_right_90_deg(&dir);
    }

    // I have documented the guards whole route
    (coords_seen, (obstacle_places.len(), hypo_count))
}



fn path_leads_to_loop(map: &Vec<Vec<bool>>, pos: &Coord, dir: &Direction, crumbs: &HashSet<BreadCrumb>, obs: &Coord) -> bool {
    // initialize Guard
    let mut pos: Coord = pos.clone();
    let mut dir: Direction = dir.clone();
    // Keep track of coords seen
    let mut crumbs: HashSet<BreadCrumb> = crumbs.clone();
    crumbs.insert(BreadCrumb{pos,dir});

    // Keep track if guard has looped
    let has_looped: bool = false;
    'guard_path: while !has_looped {
        // walk down that direction until wall hit or out of bounds
        while let Some(new_pos) = maybe_move_direction(&map, &pos, &dir, Some(obs)) {
            if crumbs.contains(&BreadCrumb{pos:new_pos, dir}) {
                return true;
            }
            // take that step
            pos = new_pos;
            crumbs.insert(BreadCrumb{pos, dir});
        }
        // Can't continue
        // if about to go out of bounds, stop tracking path;
        if check_pos_out_of_bounds(map, &pos, &dir) {break 'guard_path;}
    
        // insert crumb saying you hit a wall
        crumbs.insert(BreadCrumb{pos, dir});
        // else hit obstable, rotate
        dir = turn_right_90_deg(&dir);

        // See if I have already been here
        if crumbs.contains(&BreadCrumb{pos, dir}) {
            return true;
        }
    }
    false
}


fn turn_right_90_deg(dir: &Direction) -> Direction {
    use Direction::*;
    match dir {
        North => East,
        East => South,
        South => West,
        West => North,
    }
}

/// Return None if you can't go that direction
fn maybe_move_direction(map: &Vec<Vec<bool>>, pos: &Coord, dir: &Direction, temp_ob: Option<&Coord>) -> Option<Coord>{
    use Direction::*;
    //check bounds
    if check_pos_out_of_bounds(map, pos, dir){
        return None;
    }

    let new_pos: Coord = match dir {
        North => {Coord{row: pos.row-1, col: pos.col}},
        East => {Coord{row: pos.row, col: pos.col+1}},
        South => {Coord{row: pos.row+1, col: pos.col}},
        West => {Coord{row: pos.row, col: pos.col-1}},
    };

    if temp_ob.is_none() { // use original map
        if map[new_pos.row][new_pos.col] { None } else { Some(new_pos) }
    } else { // use original map plus this new obstacle
        let temp: Coord = Coord{row: new_pos.row, col: new_pos.col};
        if map[new_pos.row][new_pos.col] || temp == *temp_ob.unwrap() { None } else { Some(new_pos) }
    }
    
}

fn check_pos_out_of_bounds(map: &Vec<Vec<bool>>, pos: &Coord, dir: &Direction) -> bool {
   use Direction::*;
    match dir {
        North => {if pos.row == 0 {return true;}},
        East => {if pos.col == map[0].len()-1 {return true;}},
        South => {if pos.row == map.len()-1 {return true;}},
        West => {if pos.col == 0 {return true;}},
    }
    false
}


#[cfg(test)]
mod tests {
    use crate::parser::{parse_input, get_init_guard_pos};

    use super::*;

    #[test]
    fn test_p2() {
        let input = "....#.....\n....+---+#\n....|...|.\n..#.|...|.\n....|..#|.\n....|...|.\n.#.O^---+.\n........#.\n#.........\n......#...";
        let map: Vec<Vec<bool>> = parse_input(input);
        let pos = get_init_guard_pos(input).unwrap();

        let (_, count) = track_path(&map, pos);
        assert_eq!(count, (6,6));

        let input = "...\n#^#\n.#.";
        let map: Vec<Vec<bool>> = parse_input(input);
        let pos = get_init_guard_pos(input).unwrap();

        let (_, count) = track_path(&map, pos);
        assert_eq!(count, (1,1));
    
        let input = ".#.\n.^#\n.#.";
        let map: Vec<Vec<bool>> = parse_input(input);
        let pos = get_init_guard_pos(input).unwrap();

        let (_, count) = track_path(&map, pos);
        assert_eq!(count, (1,1));
    }

    #[test]
    fn test_coord_equal() {
        assert_eq!(Coord{row:10, col: 132},
                    Coord{row: 10, col: 132})
    }
}