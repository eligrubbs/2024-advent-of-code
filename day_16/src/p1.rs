/* Use Reinforcement Learning (Value Iteration) to find optimal route(s)

This solution might be a little overkill but it is what I want to use.  
There is an inverse matrix algorithm that I could use if I bothered to codify everything into matrices
and figure out some rust package.

Disocunt Factor: 1

*/
use std::env;
use std::fs::read_to_string;
use std::cmp::max;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;
use crate::grid::{Coord, Direction};


pub fn day_16_p1_soln() -> (ValueMap, Coord, i64) {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();

    let gamma: i64 = 1;
    let (mut v_map, start, end) = parse_input(&raw);

    value_iteration(&mut v_map, end, gamma);
    let cost: i64 = trace_path(&v_map, start, gamma);
    (v_map, start, cost)
}

pub fn trace_path(map: &ValueMap, start: Coord, gamma: i64) -> i64 {
    let mut path_cost: i64 = 0;

    let curr_dir: Direction = Direction::E;
    let mut curr_st: State = map.map.get(&(start, curr_dir)).unwrap().clone();

    while curr_st.typ != 'E' {
        let next_act: Action = best_action_at(map, &curr_st, gamma);
        let (next_st,cost) = map.do_action(&curr_st, &next_act);
        path_cost += cost;
        curr_st = next_st;
    }

    -1 * path_cost
}


pub fn parse_input(content: &str) -> (ValueMap, Coord, Coord) {
    let (value_map, start, end) = construct_empty_value_map(content);

    (value_map, start, end)
}

pub fn value_iteration(map: &mut ValueMap, end: Coord, gamma: i64) {
    let theta: i64 = 0; // probably have to change this

    let keys: Vec<(Coord, Direction)> = reverse_states(map, end).into_iter()
    .filter(|st| st.typ != '#' && st.typ != 'E')
    .map(|st| (st.pos, st.dir))
    .collect();

    let mut iterations = 0;
    'value_loop: loop {
        iterations += 1;
        let mut delta: i64 = 0;

        for key in keys.iter() {
            let curr_state: &State = map.map.get(&key).unwrap();
            let old_val: i64 = curr_state.val;
            let new_val: i64 = max_action_value(map, curr_state, gamma);
            map.map.get_mut(&key).unwrap().val = new_val;

            delta = max(delta,(old_val - new_val).abs());
        }
        if delta <= theta {
            break 'value_loop;
        }
    }
    println!("Iterations: {}", iterations);
}

pub fn max_action_value(map: &ValueMap, st: &State, gamma: i64) -> i64 {
    map.actions_available(st).iter().map(|act| {
        act.reward() + (gamma * map.do_action(st, act).0.val)
    }).max().unwrap()
}

pub fn action_values(map: &ValueMap, st: &State, gamma: i64) -> Vec<i64> {
    map.actions_available(st).iter().map(|act| {
        act.reward() + (gamma * map.do_action(st, act).0.val)
    }).collect()
}

pub fn best_action_at(map: &ValueMap, st: &State, gamma: i64) -> Action {
    let available: Vec<Action> = map.actions_available(st);

    let max_val: usize = available.iter().map(|act| {
        act.reward() + (gamma * map.do_action(st, act).0.val)
    }).position_max().unwrap();
    available.get(max_val).unwrap().clone()
}

fn construct_empty_value_map(content: &str) -> (ValueMap, Coord, Coord) {
    let mut value_map: HashMap<(Coord,Direction), State> = HashMap::new();
    let mut start_pos: Coord = Coord { r: 0, c: 0 };
    let mut end_pos: Coord = Coord { r: 0, c: 0 };

    for (row, line) in content.lines().enumerate() {
        for (col, char) in line.char_indices() {
            let pos: Coord = Coord::from_usize((row,col));
            if char == 'S' { start_pos = pos; }
            if char == 'E' { end_pos = pos; }
            for dir in Direction::iter() {
                let st: State = State{pos, dir, typ: char, val: 0};
                value_map.insert((pos,dir), st);
            }
        }
    }
    (ValueMap{map:value_map}, start_pos, end_pos)
}

use queues::*;
fn reverse_states(map: &ValueMap, end: Coord) -> Vec<&State> {
    let mut results: Vec<&State> = vec![];
    // load initial coords
    let mut coords_to_search: Queue<Coord> = queue![];
    let mut coords_seen: HashSet<Coord> = HashSet::new();
    for coord in non_wall_neighbors(map, end) {
        coords_to_search.add(coord).unwrap();
        coords_seen.insert(coord);
    }
    // go backwards
    while coords_to_search.size() != 0 {
        let coord_to_scan: Coord = coords_to_search.remove().unwrap();

        for dir in Direction::iter() {
            if let Some(state_to_push) = map.map.get(&(coord_to_scan, dir)){
                results.push(state_to_push);
            };
        }
        // add neighbors haven't seen
        for coord in non_wall_neighbors(map, coord_to_scan)
                                                .iter()
                                                .filter(|c| !coords_seen.contains(c))
                                                .collect::<Vec<&Coord>>() {
            coords_to_search.add(coord.clone()).unwrap();
            coords_seen.insert(coord.clone());
        }
    }

    results
}

fn non_wall_neighbors(map: &ValueMap, coord: Coord) -> Vec<Coord> {
    let mut results: Vec<Coord> = vec![];

    for dir in Direction::iter() {
        let new_coord: Coord = coord.go(dir);
        if let Some(st) =  map.map.get(&(new_coord, dir)){
            if st.typ == '.' || st.typ == 'S' {
                results.push(new_coord);
            }
        };
    }
    return results
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    N,
    E,
    S,
    W,
    TurnCw,
    TurnCCw,
}

impl Action {
    pub fn from_str(s: &str) -> Result<Action, String> {
        use Action::*;
        match s {
            "N "=> Ok(N), "E" => Ok(E), "S" => Ok(S), "W" => Ok(W),
            _ => Err(String::from(format!("Can't use {} to build an action.", s))) 
        }
    }

    pub fn from_dir(d: Direction) -> Action {
        use Action::*;
        match d {
            Direction::N => N, Direction::E => E, Direction::S => S, Direction::W => W,
        }
    }

    pub fn to_dir(&self) -> Option<Direction> {
        use Action::*;
        match self {
            N => Some(Direction::N),
            E => Some(Direction::E),
            S => Some(Direction::S),
            W => Some(Direction::W),
            _ => None,
        }
    }

    pub fn reward(&self) -> i64 {
        use Action::*;
        match self {
            N => -1, E => -1, S => -1, W => -1,
            TurnCCw => -1000, TurnCw => -1000,
        }
    }

}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    pub pos: Coord,
    pub dir: Direction,
    pub typ: char,
    pub val: i64,
}

#[derive(Debug, Clone)]
pub struct ValueMap {
    pub map: HashMap<(Coord, Direction), State>,
}


impl ValueMap {
    pub fn can_do_action(&self, st: &State, act: Action) -> bool {
        // can't do any actions at end state or wall
        if st.typ == 'E' || st.typ == '#' { return false; } 

        let new_pos: Coord = st.pos.go(act.to_dir().unwrap());
        let s_prime: Option<&State> = self.map.get(&(new_pos, st.dir));
        if let Some(state) = s_prime {
            if state.typ != '#' {
                return true;
            }
        }
        if act == Action::TurnCCw || act == Action::TurnCw {
            return true;
        }
        false
    }

    pub fn actions_available(&self, st: &State) -> Vec<Action> {
        // can't do any actions at end state or wall
        if st.typ == 'E' || st.typ == '#' { return vec![]; } 

        let mut acts: Vec<Action> = vec![Action::TurnCw, Action::TurnCCw];

        let curr_action: Action = Action::from_dir(st.dir);
        if self.can_do_action(st, curr_action) {
            acts.push(curr_action);
        }
        acts
    }

    // Return the new state of the reindeer after beginning
    pub fn do_action(&self, st: &State, act: &Action) -> (State, i64) {
        use Action::*;

        if *act != TurnCCw && *act != TurnCw {
            let new_pos: Coord = st.pos.go(act.to_dir().unwrap());

            (self.map.get(&(new_pos, st.dir)).unwrap().clone(), act.reward())
        } else {
            (self.map.get(&(st.pos, turn(st, act))).unwrap().clone(), act.reward())
        }
    }
}


// Requires that action be one of the turn actions
fn turn(st: &State, act: &Action) -> Direction {
    match act {
        Action::TurnCCw => {st.dir.turn_ccw()},
        Action::TurnCw => {st.dir.turn_cw()},
        _ => {unreachable!()}
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mdp_simple() {
        let input: &str = 
"####
#.E#
#S.#
####";
        let (mut map, start, end) = parse_input(input);
        value_iteration(&mut map, end, 1);
        let val = trace_path(&map, start, 1);
        assert_eq!(1002, val);
    }
}