use std::{env, fs::read_to_string};
use crate::p1::{Map, Dir, parse_input};

pub fn day_15_p2_soln() -> u64 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let (mut map, dirs) = parse_input(&raw);
    scale_map(&mut map);
    follow_dirs_p2(&mut map, &dirs);
    map.calc_gps()
}


fn scale_map(map: &mut Map) {
    map.map.iter_mut().for_each(|row|{
        let mut new_row = vec![];
        for chr in row.clone().iter() {
            match chr {
                '#' => {new_row.extend_from_slice(&['#','#']);},
                'O' => {new_row.extend_from_slice(&['[',']']);},
                '.' => {new_row.extend_from_slice(&['.','.']);},
                '@' => {new_row.extend_from_slice(&['@','.']);},
                _ => {unreachable!()}
            }
        }
        row.drain(0..row.len());
        row.extend(new_row);
    });
    map.robot_pos = (map.robot_pos.0, map.robot_pos.1*2);
    return;
}

fn follow_dirs_p2(map: &mut Map, dirs: &Vec<Dir>) {
    for dir in dirs {
        map.try_move_p2(dir);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scale_map() {
        let input1: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";
        let output: &str = "####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################";

        let (mut map, _) = parse_input(input1);
        scale_map(&mut map);
        assert_eq!(map.print(), output);
    }

    #[test]
    fn test_p2_spec() {
        let input1: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let (mut map, dirs) = parse_input(input1);
        scale_map(&mut map);
        follow_dirs_p2(&mut map, &dirs);
        map.print();
        assert_eq!(9021, map.calc_gps());
    }

    #[test]
    fn test_simple_p2_movement() {
        let input: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let (mut map, dirs) = parse_input(input);
        scale_map(&mut map);
        follow_dirs_p2(&mut map, &dirs);
    }
}