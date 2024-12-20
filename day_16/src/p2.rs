use std::collections::HashSet;
use std::iter::zip;

use crate::grid::{Direction, Coord};
use crate::p1::{ValueMap, State, max_action_value, action_values};


pub fn day_16_p2_soln(map: &ValueMap, start: Coord, gamma: i64) -> i64 {
    spots_from_all_paths(map, start, gamma) as i64
}

pub fn spots_from_all_paths(map: &ValueMap, start: Coord, gamma: i64) -> usize {
    let mut best_spots: HashSet<Coord> = HashSet::new();
    let mut curr_dir: Direction = Direction::E;
    let mut curr_pos: Coord = start;

    // let mut curr_st: State = map.map.get(&(curr_pos, curr_dir)).unwrap().clone();
    let mut keys_to_check: HashSet<(Coord, Direction)> = HashSet::from([(start, curr_dir)]);

    while keys_to_check.len() != 0 {
        // remove an element from keys_to_check
        if let Some((pos, dir)) = keys_to_check.iter().nth(0){
            (curr_pos, curr_dir) =  (pos.clone(), dir.clone());
        };
        keys_to_check.remove(&(curr_pos,curr_dir));

        // say that I have seen this coordinate
        best_spots.insert(curr_pos.clone());

        // add all possible best actions to keys_to_check
        let curr_st: &State = map.map.get(&(curr_pos, curr_dir)).unwrap();
        let max_val: i64 = max_action_value(map, curr_st, gamma);
        zip(map.actions_available(curr_st).iter(), action_values(map, curr_st, gamma)).for_each(|(act, act_val)| {
            if act_val == max_val {
                let (next_st, _) = map.do_action(curr_st, act);
                keys_to_check.insert((next_st.pos, next_st.dir));
            }
        });
    }
    // while curr_st.typ != 'E' {
    //     let next_act: Action = best_action_at(map, &curr_st, gamma);
    //     let (next_st,_) = map.do_action(&curr_st, &next_act);

    //     curr_st = next_st;
    // }

    best_spots.len()
}