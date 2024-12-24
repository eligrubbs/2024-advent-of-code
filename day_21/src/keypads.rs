/*
Contains the human and directional keypads.

*/
use std::fmt;
use strum_macros::EnumIter;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DirectionalPad {
    pos: Coord,
}

impl DirectionalPad {

    pub fn from_char(c: char) -> Self {
        let pos: Coord = Self::char_pos(c);
        Self{pos}
    }

    pub fn char_pos(c: char) -> Coord {
        assert!(c == '^' || c == 'v' || c == '>' || c == '<' || c == 'A');

        let pos: (i32, i32) = match c {
                          '^' => (1,1), 'A' => (1,2),
            '<' => (0,0), 'v' => (0,1), '>' => (0,2),
            _ => unreachable!(),
        };
        Coord::from(pos)
    }

    pub fn to_char(coord: &Coord) -> char {
        match (coord.r, coord.c) {
                          (1,1) => '^', (1,2) => 'A',
            (0,0) => '<', (0,1) => 'v', (0,2) => '>',
            _ => unreachable!(),
        }
    }

    /// Move this current pad's position to the char `c` on the pad.
    /// 
    /// INCLUDES THE 'A'
    /// 
    /// Always safe to move East or South.
    pub fn move_to(&mut self, c: char) -> String {
        let dest: Coord = Self::char_pos(c);

        let mut dirs: String = String::new();

        let vector: Coord = self.pos.diff(&dest);

        // decide if I need to move west and I wont get into the bad spot
        if vector.c < 0 && (dest.c != 0 && self.pos.r == 1) {
            for _ in 0..(-1*vector.c) { dirs.push('<'); }
        }
        // decide if I need to move south
        if vector.r < 0 {
            for _ in 0..(-1*vector.r) { dirs.push('v'); }
        }
        // decide if I need to move east
        if vector.c > 0 {
            for _ in 0..(vector.c) { dirs.push('>'); }
        }
        // decide if I need to move north
        if vector.r > 0 {
            for _ in 0..vector.r { dirs.push('^'); }
        }

        // decide if I need to move west and the first branch didn't trigger
        if vector.c < 0 && !(dest.c != 0 && self.pos.r == 1){
            for _ in 0..(-1*vector.c) { dirs.push('<'); }
        }


        dirs.push('A');
        // println!("From: {:?} To {:?} by {:?} yields {}", self.pos, dest, vector, dirs);
        self.pos = dest;

        dirs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumberPad {
    pos: Coord,
}

impl NumberPad {
    pub fn from_char(c: char) -> Self {
        let pos: Coord = Self::char_pos(c);
        Self{pos}
    }

    pub fn char_pos(c: char) -> Coord {
        assert!(c.is_digit(10) || c == 'A');

        let pos: (i32, i32) = match c {
            '7' => (3,0), '8' => (3,1), '9' => (3,2),
            '4' => (2,0), '5' => (2,1), '6' => (2,2),
            '1' => (1,0), '2' => (1,1), '3' => (1,2),
                          '0' => (0,1), 'A' => (0,2),
            _ => unreachable!(),
        };
        Coord::from(pos)
    }

    pub fn to_char(coord: &Coord) -> char {
        match (coord.r, coord.c) {
            (3,0) => '7', (3,1) => '8', (3,2) => '9',
            (2,0) => '4', (2,1) => '5', (2,2) => '6',
            (1,0) => '1', (1,1) => '2', (1,2) => '3',
                          (0,1) => '0', (0,2) => 'A',
            _ => unreachable!(),
        }
    }

    /// move position of the current pad to the char passed in.
    /// Returns the string of directions it took. 
    ///
    /// INCLUDES THE 'A' (which represents a press)
    /// 
    /// always safe to move east or north.
    pub fn move_to(&mut self, c: char) -> String {
        let dest: Coord = Self::char_pos(c);

        let mut dirs: String = String::new();

        let vector: Coord = self.pos.diff(&dest);

        if vector.c < 0 && (dest.c != 0 && self.pos.r != 0 ){
            for _ in 0..(-1*vector.c) { dirs.push('<'); }
        }
        // decide if I need to move south
        if vector.r < 0 && (dest.r != 0 && self.pos.c != 0) {
            for _ in 0..(-1*vector.r) { dirs.push('v'); }
        }

        // decide if I need to move east
        if vector.c > 0 {
            for _ in 0..(vector.c) { dirs.push('>'); }
        }
        // decide if I need to move north
        if vector.r > 0 {// move up
            for _ in 0..vector.r { dirs.push('^'); }
        }
        // decide if I need to move west

        if vector.c < 0 && !(dest.c != 0 && self.pos.r != 0 ){
            for _ in 0..(-1*vector.c) { dirs.push('<'); }
        }
        // decide if I need to move south
        if vector.r < 0 && !(dest.r != 0 && self.pos.c != 0) {
            for _ in 0..(-1*vector.r) { dirs.push('v'); }
        }

        dirs.push('A');

        // println!("NUM: From: {:?} To {:?} by {:?} yields {}", self.pos, dest, vector, dirs);
        self.pos = dest;
        dirs
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Assumes c is column based and that r is row based.
/// Assumes coordinates in 4th quadrant and 4th only (plus axes)
pub struct Coord {
    pub r: i32,
    pub c: i32,
}

impl Coord {
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

    pub fn from(pos: (i32, i32)) -> Coord {
        Coord{r:pos.0,c: pos.1}
    }

    /// returns a coordinate with (`other`.r- `self`.r, `other`.c - `self`.c)
    /// 
    /// Represents a vector that when followed from `self` takes you to `other`.
    pub fn diff(&self, other: &Coord) -> Coord {
        Coord::from((other.r - self.r, other.c - self.c))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    N, E, S, W
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }

}

impl Direction {
    pub fn to_str(&self) -> &str {
        use Direction::*;
        match self {
            N => "^", E => ">", S => "v", W => "<",
        }
    }
    pub fn from(s: &str) -> Direction {
        use Direction::*;
        match s {
            "^" => N, ">" => E, "v" => S, "<" => W,
            _ => unreachable!(),
        }
    }
}