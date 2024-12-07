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
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BreadCrumb {
    pos: Coord,
    dir: Direction,
}

pub fn day_6_p1_soln() -> usize {
    let (orig_map, pos) = parse_day_6_input();
    let orig_pos: Coord = Coord{row:pos.0.try_into().unwrap(), col:pos.1.try_into().unwrap()};
    let orig_dir: Direction = Direction::North;

    let (_, places_seen, _) = does_map_loop(&orig_map, &orig_pos, &orig_dir, false);

    places_seen.len()
}

pub fn day_6_p2_soln() -> usize {
    let (orig_map, pos) = parse_day_6_input();
    let orig_pos: Coord = Coord{row:pos.0.try_into().unwrap(), col:pos.1.try_into().unwrap()};
    let orig_dir: Direction = Direction::North;

    // for every position, understand if the spot infront of it will make it loop or not.
    let (_, _, potential_loop_obstacles) = does_map_loop(&orig_map, &orig_pos, &orig_dir, true);

    potential_loop_obstacles.len()
}


/// Given a map and a original position, move around until either out of bounds, or on same path.
fn does_map_loop(map: &Vec<Vec<bool>>, start_pos: &Coord, start_dir: &Direction, is_main: bool) -> (bool, HashSet<Coord>, HashSet<Coord>) {
    let mut pos: Coord = start_pos.clone();
    let mut dir: Direction = start_dir.clone();

    let mut steps_taken: HashSet<BreadCrumb> = HashSet::new();
    let mut places_been: HashSet<Coord> = HashSet::new();
    let mut potential_loop_obstacles: HashSet<Coord> = HashSet::new();

    let mut does_loop = false;

    'map_runner: while coord_in_bounds(map, &pos) {
        let curr_step: BreadCrumb = BreadCrumb{pos:pos, dir:dir};

        // See if I have been here before
        if steps_taken.contains(&curr_step) {
            does_loop = true;
            break 'map_runner;
        }
        // Add this to steps I've taken
        steps_taken.insert(curr_step);
        places_been.insert(pos);


        let (temp_pos, temp_dir) = take_step(&map, &pos, &dir);

        // I could have turned (temp_pos same), or I could have moved forward (temp_pos different)
        // If I am about to move forward, check if it would loop of this was an obstacle
        // Only do this recursive call if I am main caller. Also, it can't be the first position.
        // Can't place an obstacle in a position that the guard has already walked onto.
        if temp_pos != pos && is_main && coord_in_bounds(map, &temp_pos) && !places_been.contains(&temp_pos){
            // Convert my next step into an obstacle
            let mut temp_map: Vec<Vec<bool>> = map.clone();
            temp_map[i32_to_usize(temp_pos.row)][i32_to_usize(temp_pos.col)] = true;

            if does_map_loop(&temp_map, &pos, &dir, false).0 {
                potential_loop_obstacles.insert(temp_pos);
            }
        }

        // move forward
        (pos,dir) = (temp_pos, temp_dir);
    }

    (does_loop, places_been, potential_loop_obstacles)
}


/// Given a map and a position, take a step forward, which if infront of a wall, means turning and not moving
/// Assumes that start_pos is inbounds!
/// 
/// Makes no assumption about if the returned position is in-bounds or not.
fn take_step(map: &Vec<Vec<bool>>, start_pos: &Coord, start_dir: &Direction) -> (Coord, Direction) {
    let pos_infront: Coord = move_coord_in_dir(start_pos, start_dir);

    // chance that I just moved out of bounds
    let mut obstacle_infront: bool = false;
    if coord_in_bounds(map, &pos_infront) {
        let map_row: &Vec<bool> = &map[i32_to_usize(pos_infront.row)];
        obstacle_infront = map_row[i32_to_usize(pos_infront.col)];
    }

    if obstacle_infront { // if there is something directly infront of you, turn right 90 degrees
        return (start_pos.clone(), turn_90_degrees(&start_dir));
    } else { // else take a step forward, don't change direction
        (pos_infront, start_dir.clone())
    }
}

fn turn_90_degrees(dir: &Direction) -> Direction {
    use Direction::*;
    match dir { North => East, East => South, South => West, West => North }
}

/// Update coordination based on direction given. 
/// 
/// Does not impose any limits on map boundaries. Can return out of bounds coordinate.
fn move_coord_in_dir(pos: &Coord, dir: &Direction) -> Coord {
    use Direction::*;
    match dir {
        North => {Coord{row: pos.row-1, col: pos.col  }},
        East  => {Coord{row: pos.row,   col: pos.col+1}},
        South => {Coord{row: pos.row+1, col: pos.col  }},
        West  => {Coord{row: pos.row,   col: pos.col-1}},
    }
}

/// Return true if the coordinate is inbounds of the map
fn coord_in_bounds(map: &Vec<Vec<bool>>, pos: &Coord) -> bool {
    (0 <= pos.row && pos.row < map.len().try_into().unwrap()) && (0 <= pos.col && pos.col < map[0].len().try_into().unwrap())
}

fn i32_to_usize(x: i32) -> usize {
    <i32 as TryInto<usize>>::try_into(x).unwrap()
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
        let pos: Coord = Coord{row: pos.0.try_into().unwrap(), col: pos.1.try_into().unwrap()};

        let (_, _, bob) = does_map_loop(&map, &pos, &Direction::North, true);
        assert_eq!(bob.len(), 6);

        let input = "...\n#^#\n.#.";
        let map: Vec<Vec<bool>> = parse_input(input);
        let pos = get_init_guard_pos(input).unwrap();
        let pos: Coord = Coord{row: pos.0.try_into().unwrap(), col: pos.1.try_into().unwrap()};

        let (_, _, bob) = does_map_loop(&map, &pos, &Direction::North, true);
        assert_eq!(bob.len(), 1);
    
        let input = ".#.\n.^#\n.#.";
        let map: Vec<Vec<bool>> = parse_input(input);
        let pos = get_init_guard_pos(input).unwrap();
        let pos: Coord = Coord{row: pos.0.try_into().unwrap(), col: pos.1.try_into().unwrap()};
        
        let (_, _, bob) = does_map_loop(&map, &pos, &Direction::North, true);
        assert_eq!(bob.len(), 1);
    }

    #[test]
    fn test_coord_equal() {
        assert_eq!(Coord{row:10, col: 132},
                    Coord{row: 10, col: 132})
    }
}