use std::{collections::{HashSet, HashMap}, fmt::{self, Debug}};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[derive(Debug, Clone)]
pub struct Grid {
    pub map: HashMap<Coord, u32>,
    pub typs: HashMap<Coord, char>,
    pub start: Coord,
    pub end: Coord,
}

impl Grid {
    pub fn get_neighbors(&self, coord: &Coord) -> HashSet<(Coord, Direction)> {
        let mut result: HashSet<(Coord, Direction)> = HashSet::new();
        for dir in Direction::iter() {
            let neighbor: Coord = coord.go(dir);
            if let Some(_neigh) = self.typs.get(&neighbor) {
                result.insert((neighbor, dir));
            }
        }
        result
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    N, E, S, W
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

    pub fn opp(&self) -> Direction {
        use Direction::*;
        match self {
            N => S, E => W, S => N, W => E
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Assumes c is column based and that r is row based.
/// Assumes coordinates in 4th quadrant and 4th only (plus axes)
pub struct Coord {
    pub r: usize,
    pub c: usize,
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

    pub fn from(tup: (usize, usize)) -> Coord {
        Coord{r: tup.0, c: tup.1}
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

    pub fn manhat_dist(&self, other: &Coord) -> usize {
        self.r.abs_diff(other.r) + self.c.abs_diff(other.c)
    }
}