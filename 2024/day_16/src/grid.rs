use std::fmt::{self, Debug};
use strum_macros::EnumIter;

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