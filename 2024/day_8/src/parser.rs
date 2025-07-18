use std::{collections::HashMap, env, fs::read_to_string};

/// Integer coordinates in x,y plane.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coord {
    pub r: i32,
    pub c: i32,
}

pub fn day_8_parse_input() -> (HashMap<char, Vec<Coord>>, (i32,i32)) {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    parse_input(&raw)
}

/// Take in sattelite position raw string
/// and return HashMap of coord -> char as well as the bounds (line number -1, chars per line -1)
fn parse_input(content: &str) -> (HashMap<char, Vec<Coord>>, (i32,i32)){
    let mut char_map: HashMap<char, Vec<Coord>> = HashMap::new();

    let num_rows: i32 = (content.lines().collect::<Vec<&str>>().len()-1) as i32;
    let num_cols: i32 = (content.lines().collect::<Vec<&str>>()[0].len()-1) as i32;

    for (row, line) in content.lines().enumerate() {
        for (col, chr) in line.char_indices() {
            if chr == '.' {
                continue;
            }
            let antennas_of_type_chr: &mut Vec<Coord> = char_map.entry(chr).or_insert(vec![]);
            antennas_of_type_chr.push(Coord{r:row as i32, c:col as i32});
        }
    }
    (char_map, (num_rows,num_cols))
}

