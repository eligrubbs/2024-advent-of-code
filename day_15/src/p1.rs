use std::{collections::HashSet, env, fs::read_to_string};

pub fn day_15_p1_soln() -> u64 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let (mut map, dirs) = parse_input(&raw);
    follow_dirs(&mut map, &dirs);
    map.calc_gps()
}


pub fn follow_dirs(map: &mut Map, dirs: &Vec<Dir>) {
    for dir in dirs {
        map.try_move(dir);
    }
}

pub fn parse_input(content: &str) -> (Map, Vec<Dir>){
    let mut map_section: bool = true;
    let mut found_rob: bool = false;
    let mut rob_pos: (usize, usize) = (0,0);
    let mut map: Vec<Vec<char>> = vec![];
    let mut directions: Vec<Dir> = vec![];

    content.lines().enumerate().for_each(
        |(row, line)| {
            if line.is_empty() {
                map_section = false;
            } else {
                if map_section {
                    map.push(line.char_indices().map(|(col, chr)| {
                        if chr=='@' { found_rob=true; rob_pos=(row,col); } chr
                    }).collect::<Vec<char>>());
                    
                } else {
                    line.chars()
                        .for_each(|chr|{
                            directions.push(Dir::from(chr)); });
                }
            }
    });
    assert!(found_rob);
    (Map{map, robot_pos: rob_pos}, directions)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    N, E, S, W
}

impl Dir {
    fn from(chr: char) -> Dir {
        use Dir::*;
        match chr {
            '^' => {N},
            '>' => {E},
            '<' => {W},
            'v' => {S},
            _ => {unreachable!()}
        }
    }
    pub fn offset(&self) -> (i16,i16) {
        use Dir::*;
        match self {
            N => {(-1,0)},
            E => {(0,1)},
            S => {(1,0)},
            W => {(0,-1)},
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub map: Vec<Vec<char>>,
    pub robot_pos: (usize, usize),
}

impl Map {
    pub fn print(&self) -> String {
        let mut output: String = String::new();
        self.map.iter().for_each(|row| {
            row.iter().for_each(|&chr| output.push(chr));
            output.push('\n');
        });
        output.push_str(&format!("{:?}", self.robot_pos));
        output
    }

    /// Attemps to move the robot,
    pub fn try_move(&mut self, dir: &Dir) -> bool {
        // Gather the free space
        let offset: (i16, i16) = dir.offset();

        // attempt a move
        if let Some(dot_pos) = self.scan_dir_for_opening(offset) {
            if *dir == Dir::N || *dir == Dir::S {
                self.robot_pos = self.shift_up_down_from_pos(dot_pos, self.robot_pos, dir);
            } else {
                self.robot_pos = self.shift_left_right_from_pos(dot_pos, self.robot_pos, dir);
            }
            true
        } else {
            false
        }
    }

    /// Attempts to move according to p2 logic.
    pub fn try_move_p2(&mut self, dir: &Dir) -> bool {

        if *dir == Dir::N || *dir == Dir::S {
            return self.p2_up_down_helper(dir)
        }else {//p1 logic for East and West
            return self.try_move(dir)
        }

    }

    fn p2_up_down_helper(&mut self, dir: &Dir) -> bool {
        let mut did_move = false;
        let mut positions: HashSet<(usize,usize)> = HashSet::from([self.robot_pos]);
        let mut order_to_move: Vec<HashSet<(usize,usize)>> = vec![positions.clone()];
        while !self.dots_past_all_in_dir(&positions, dir) && !self.wall_past_one_in_dir(&positions, dir){// while there are still blocks in front of me
            positions = self.find_all_blocks_in_dir(&positions, dir);
            order_to_move.push(positions.clone());
        }
        let mut new_robot_pos: (usize,usize) = self.robot_pos;
        for ind_dudes_to_move in (0..order_to_move.len()).rev() {
            // only do the move if there are only dots infront of all the dudes
            if self.dots_past_all_in_dir(&order_to_move[ind_dudes_to_move], dir) {
                did_move = true;
                for dude_to_move in order_to_move[ind_dudes_to_move].clone() {
                    // it is fine that I always set here, because last one to set will be original robot
                    new_robot_pos = self.shift_up_down_from_pos(add_pos(dude_to_move, dir.offset()), dude_to_move, dir);
                }
            }
        }
        //update robot
        self.robot_pos = new_robot_pos;
        did_move
    }

    /// returns true if all the positions 1 step of dir from all positions passed in are '.'s.
    /// false otherwise
    fn dots_past_all_in_dir(&self, positions: &HashSet<(usize,usize)>, dir: &Dir) -> bool {
        let offset: (i16, i16) = dir.offset();
        for pos in positions {
            if self.char_at(add_pos(*pos, offset)) != '.' {
                return false;
            }
        }
        true
    }
    /// returns true if at least 1 of the positions 1 step of dir from all positions passed in ends up being #.
    /// false otherwise
    fn wall_past_one_in_dir(&mut self, positions: &HashSet<(usize,usize)>, dir: &Dir) -> bool {
        let offset: (i16, i16) = dir.offset();
        for pos in positions {
            if self.char_at(add_pos(*pos, offset)) == '#' {
                return true;
            }
        }
        false
    }


    /// assumes that the current row has no #'s and it not all dots
    fn find_all_blocks_in_dir(&self, positions: &HashSet<(usize,usize)>, dir: &Dir) -> HashSet<(usize,usize)> {
        let offset: (i16, i16) = dir.offset();
        let mut result: HashSet<(usize,usize)> = HashSet::new();
        for pos in positions {
            let possible_box = add_pos(*pos, offset);
            if self.char_at(possible_box) == '[' {
                result.insert(possible_box);
                result.insert((possible_box.0, possible_box.1+1));
            }
            if self.char_at(possible_box) == ']' {
                result.insert(possible_box);
                result.insert((possible_box.0, possible_box.1-1));
            }
        }
        result
    }

    /// Look for position of first '.' char in this direction, stopping when you hit a '#' char.
    /// Returns None if a '#' is found before a '.' is found
    fn scan_dir_for_opening(&self, offset: (i16,i16)) -> Option<(usize,usize)> {
        let mut dot_pos: Option<(usize,usize)> = None;
        let mut offset_m: (i16, i16) = offset;
        let mut next_char: char = self.char_at(self.robot_offset(offset_m) );

        while next_char != '#' {
            if next_char == '.' {
                dot_pos = Some(add_pos(self.robot_pos, offset_m));
                break;
            };

            offset_m = (offset_m.0 + offset.0, offset_m.1 + offset.1);
            next_char = self.char_at(self.robot_offset(offset_m));
        }

        dot_pos
    }

    fn robot_offset(&self, offset: (i16,i16)) -> (usize, usize) {
        add_pos(self.robot_pos, offset)
    }

    fn char_at(&self, pos: (usize, usize)) -> char {
        self.map[pos.0][pos.1]
    }

    fn shift_left_right_from_pos(&mut self, dot_pos: (usize,usize), start_pos: (usize,usize), dir: &Dir) -> (usize,usize) {
        let new_pos: (usize, usize) = add_pos(start_pos, dir.offset());

        let row: &mut Vec<char> = self.map.get_mut(dot_pos.0).unwrap();
        row.remove(dot_pos.1);
        row.insert(start_pos.1, '.');

        new_pos
    }

    /// I know dot_pos is above robot
    fn shift_up_down_from_pos(&mut self, dot_pos: (usize,usize), start_pos: (usize,usize), dir: &Dir) -> (usize, usize) {
        let new_robot_pos: (usize, usize) = add_pos(start_pos, dir.offset()); //self.robot_offset(dir.offset());
        
        // I use X's because they should not exist at the end, will help me catch bugs
        if *dir == Dir::S { // pull dot up to robot_pos if dot is south
            // assert that dot really is below robot
            assert!(dot_pos.1 == start_pos.1 && start_pos.0 < dot_pos.0);
            for swap_row in (start_pos.0+1..dot_pos.0+1).rev() {
                self.map[swap_row][dot_pos.1] = self.map[swap_row-1][dot_pos.1]; // pull other char down
                self.map[swap_row-1][dot_pos.1] = 'X'; // bubble dot up
            }
            
        } else { // assert that dot really is above robot 
            // push the dot down to robot_pos
            assert!(dot_pos.1 == start_pos.1 && start_pos.0 > dot_pos.0);
            for swap_row in dot_pos.0..start_pos.0 {
                self.map[swap_row][dot_pos.1] = self.map[swap_row+1][dot_pos.1]; // bubble other char up
                self.map[swap_row+1][dot_pos.1] = 'X'; //push dot char down
            }
        }
        // put dot in position where robot was
        self.map[start_pos.0][start_pos.1] = '.';
        new_robot_pos
    }

    pub fn calc_gps(&self) -> u64 {
        let mut gps: u64 = 0;
        self.map.iter().enumerate().for_each(|(r_ind, row)| {
            row.iter().enumerate().for_each(|(c_ind, &chr)| {
                if chr == 'O' || chr == '['{
                    gps += 100*(r_ind as u64) + (c_ind as u64);
                }
            });
        });
        gps
    }
}

fn add_pos(pos: (usize, usize), offset: (i16,i16)) -> (usize, usize) {
    let (r,c) = (pos.0 as i16, pos.1 as i16);
    let (r,c) = (r+offset.0, c+offset.1);
    (r as usize, c as usize)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gps() {
        let input: &str = "#######
#...O..
#.....@";
        let (map,_) = parse_input(input);
        assert_eq!(map.calc_gps(), 104);
    }

    #[test]
    fn test_spec_small() {
        let input: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let (mut map, dirs) = parse_input(input);
        follow_dirs(&mut map, &dirs);

        let input2: &str = "########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########";
        let (map_2,_) = parse_input(input2);

        assert_eq!(map, map_2);
        assert_eq!(2028, map.calc_gps());
    }
}