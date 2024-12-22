use std::{collections::HashSet, fmt::{self, Debug}};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use itertools::Itertools;


#[derive(Debug, Clone)]
pub struct Grid {
    pub blocks: Vec<Vec<bool>>,
    pub value_map: Vec<Vec<i64>>,
    pub width: usize,
    pub height: usize,
    pub end_state: Coord
}

impl Grid {
    pub fn from_blocks(blocks: &HashSet<Coord>, width: usize, height: usize) -> Grid {
        let mut g: Vec<Vec<bool>> = vec![];
        for row in 0..(height as usize) {
            g.push(vec![]);
            for col in 0..(width as usize) {
                g[row].push( blocks.contains(&Coord::from((row as i32, col as i32))) );
            }
        }

        let value_map: Vec<Vec<i64>> = vec![vec![0;width];height];
        Grid{blocks:g, value_map, width, height, end_state: Coord::from_usize((height-1, width-1))}
    }

    pub fn get_valid_actions_and_next_states(&self, coord: &Coord) -> Vec<(Direction, Coord)> {
        let mut result: Vec<(Direction, Coord)> = vec![];
        if *coord == self.end_state ||
            self.blocks[coord.r as usize][coord.c as usize] {
                // end state and walls have no actions
                return vec![]
            }

        for dir in Direction::iter() {
            let next_pos: Coord = coord.go(dir);
            if self.pos_in_bounds(&next_pos) && !self.blocks[next_pos.r as usize][next_pos.c as usize] {
                result.push((dir, next_pos));
            }
        }
        result
    }

    pub fn pos_in_bounds(&self, coord: &Coord) -> bool {
        coord.r >= 0 && coord.r < (self.height as i32) && coord.c >= 0 && coord.c < (self.width as i32)
    }

    pub fn action_values_at(&self, coord: &Coord, gamma: i64) -> Vec<i64> {
        let acts_and_s_prime: Vec<(Direction, Coord)> = self.get_valid_actions_and_next_states(coord);
        acts_and_s_prime.iter().map(|(_, c_new)| {
            -1 + (gamma * self.value_map[c_new.r as usize][c_new.c as usize])
        }).collect()
    }

    ///returns the maximum action value from the actions that can be taken or 0 if this state has no available actions
    pub fn max_action_value(&self, coord: &Coord, gamma: i64) -> i64 {
        self.action_values_at(coord, gamma).iter().max().unwrap_or(&0).clone()
    }

    pub fn best_action_and_next_pos_at(&self, coord: &Coord, gamma: i64) -> (Direction, Coord) {
        let acts_and_s_prime: Vec<(Direction, Coord)> = self.get_valid_actions_and_next_states(coord);
        let pos_best_action: usize = self.action_values_at(coord, gamma).iter().position_max().unwrap(); 
        acts_and_s_prime[pos_best_action].clone()
    }

    pub fn map_str(&self) -> String {
        let mut res: String = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                res.push(if self.blocks[row][col] {'#'} else {'.'} );
            }
            res.push('\n');
        }
        res
    }

    pub fn map_str_steps(&self, steps: &Vec<Coord>) -> String {
        let mut res: String = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                res.push(if self.blocks[row][col] {'#'}
                             else if steps.contains(&Coord::from_usize((row,col))) {'O'}
                             else if steps.contains(&Coord::from_usize((row,col))) && self.blocks[row][col] { panic!() }
                             else {'.'} );
            }
            res.push('\n');
        }
        res
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    N,
    E,
    S,
    W
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl Direction {
    /// Return direction as &str
    pub fn str(&self) -> String {
        use Direction::*;
        (match self { N => "N", E => "E", S => "S", W => "W", }).to_string()
    }
    pub fn from(s: &str) -> Result<Direction, String> {
        use Direction::*;
        match s {
            "N "=> Ok(N), "E" => Ok(E), "S" => Ok(S), "W" => Ok(W),
            _ => Err(String::from(format!("Can't use {} to build a direction.", s))) 
        }
    }

    pub fn turn_cw(&self) -> Direction {
        use Direction::*;
        match self {
            N => E, E => S, S => W, W => N
        }
    }

    pub fn turn_ccw(&self) -> Direction {
        use Direction::*;
        match self {
            N => W, E => N, S => E, W => S
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Assumes c is column based and that r is row based.
/// Assumes coordinates in 4th quadrant and 4th only (plus axes)
pub struct Coord {
    pub r: i32,
    pub c: i32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl Coord {

    /// returns coordinate as a string in `(row,col)` format.
    pub fn str(&self) -> String {
        format!("({},{})", self.r, self.c)
    }

    pub fn from(tup: (i32, i32)) -> Coord {
        Coord{r: tup.0, c: tup.1}
    }

    pub fn from_usize(tup: (usize, usize)) -> Coord {
        Coord{r: tup.0 as i32, c: tup.1 as i32}
    }

    /// Creates a new coordinate 1 step in the direction of `dir`.
    /// This may be outside the 4th coordinate, so check bounds of the returned coordinate.
    pub fn go(&self, dir: Direction) -> Coord {
        use Direction::*;
        match dir {
            N => Coord::from((self.r-1, self.c)),
            E => Coord::from((self.r, self.c+1)),
            S => Coord::from((self.r+1, self.c)),
            W => Coord::from((self.r, self.c-1)),
        }
    }
}