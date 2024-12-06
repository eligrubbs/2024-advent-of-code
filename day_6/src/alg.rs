use std::collections::HashSet;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Corner {
    pos: Coord,
    typ: Direction
}


pub fn day_6_p1_soln() -> usize {
    let (map, init_pos) = parse_day_6_input();
    let (crumbs, _) = track_path(&map, init_pos);

    crumbs.len()
}


fn track_path(map: &Vec<Vec<bool>>, init_pos: (usize,usize)) -> (HashSet<Coord>, HashSet<Corner>){

    // initialize Guard
    let mut pos: Coord = Coord{row: init_pos.0, col: init_pos.1};
    let mut dir: Direction = Direction::North;

    // Keep track of coords seen
    let mut coords_seen: HashSet<Coord> = HashSet::from([pos]);
    let mut crumbs: HashSet<BreadCrumb> = HashSet::from([BreadCrumb{pos,dir}]);
    let mut corners: HashSet<Corner> = HashSet::new();
    // Keep track if guard has looped
    let has_looped: bool = false;

    'guard_path: while !has_looped {
        //If I am about to start on a path I already have, break
        match maybe_move_direction(&map, &pos, &dir) {
            Some(next_pos) => {
                if crumbs.contains(&BreadCrumb{pos:next_pos, dir}) {
                    // has_looped = true;
                    break 'guard_path;
                }
            },
            None => {}, // something infront of me, I'll rotate eventually
        };

        // continue until you hit a block or are out of bounds
        while let Some(new_pos) = maybe_move_direction(&map, &pos, &dir) {
            // take that step
            pos = new_pos;
            coords_seen.insert(pos);
            crumbs.insert(BreadCrumb{pos, dir});
        }
        // Can't continue
        // if about to go out of bounds, stop tracking path;
        use Direction::*;
        match dir {
            North => {if pos.row == 0 {break 'guard_path;}},
            East => {if pos.col == map[0].len()-1 {break 'guard_path;}},
            South => {if pos.row == map.len()-1 {break 'guard_path;}},
            West => {if pos.col == 0 {break 'guard_path;}},
        }
    
        // else hit obstable, log this obstacle as a corner and rotate
        corners.insert(Corner{pos, typ:dir});
        dir = turn_right_90_deg(&dir);
    }

    // I have documented the guards whole route
    (coords_seen, corners)
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
fn maybe_move_direction(map: &Vec<Vec<bool>>, pos: &Coord, dir: &Direction) -> Option<Coord>{
    use Direction::*;
    //check bounds
    match dir {
        North => {if pos.row == 0 {return None;}},
        East => {if pos.col == map[0].len()-1 {return None;}},
        South => {if pos.row == map.len()-1 {return None;}},
        West => {if pos.col == 0 {return None;}},
    }

    let new_pos: Coord = match dir {
        North => {Coord{row: pos.row-1, col: pos.col}},
        East => {Coord{row: pos.row, col: pos.col+1}},
        South => {Coord{row: pos.row+1, col: pos.col}},
        West => {Coord{row: pos.row, col: pos.col-1}},
    };

    if map[new_pos.row][new_pos.col] {
        None
    } else {
        Some(new_pos)
    }
} 