use std::env;
use std::fs::read_to_string;


/// Enum `ReadDirection` stores a direction that text was read in.
/// 
/// # Examples
/// 
///  111  
///  222  
///  333  
/// 
/// ### Read from:
/// GoNorth: 321  
/// GoSouth: 123  
/// GoNorthWest: 321  
/// GoSouthWest: 123  
/// etc.
pub enum ReadDirection {
    GoNorth=1,
    GoNorthEast=2,
    GoEast=3,
    GoSouthEast=4,
    GoSouth=5,
    GoSouthWest=6,
    GoWest=7,
    GoNorthWest=8,
}


pub fn parse_direction(dir: ReadDirection) -> Vec<String>{
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let file_str: String = read_to_string(path.to_str().unwrap()).unwrap();

    lists_by_dir(&file_str, dir)
}


/// Create a list for every line in `content` based on `ReadDirection`
/// 
/// `content`: string with newlines containing all the data
///     all lines in content assumed to be same length
///  `dir`: a `ReadDirection` which determines how the content is returned
/// 
/// ## Returns
/// A list of strings with order changed to that of `dir`.
/// Note, if dir is diagonal, each list will not be same length
pub fn lists_by_dir(content: &str, dir: ReadDirection) -> Vec<String> {
    use ReadDirection::*;

    match dir {
        GoNorth => {build_north_dir(content)},
        GoNorthEast => {build_north_east_dir(content)},
        GoEast => {build_east_dir(content)},
        GoSouthEast => {build_south_east_dir(content)},
        GoSouth => {build_south_dir(content)},
        GoSouthWest => {build_south_west_dir(content)},
        GoWest => {build_west_dir(content)},
        GoNorthWest => {build_north_west_dir(content)},
    }
}


fn build_east_dir(content: &str) -> Vec<String> {
    content.lines().map(|x| String::from(x)).collect()
}

fn build_west_dir(content: &str) -> Vec<String> {
    build_east_dir(content).iter().map(|x| x.chars().rev().collect::<String>()).collect()
}


fn build_north_dir(content: &str) -> Vec<String> {
    let base: Vec<String> = build_east_dir(content);

    let mut result: Vec<String> = vec!["".to_string(); base.len()];
    for x in base.iter() {
        for (i, c) in x.char_indices() {
            result.get_mut(i).unwrap().push(c)
        }
    }

    result
}

fn build_south_dir(content: &str) -> Vec<String> {
    build_north_dir(content).iter().map(|x| x.chars().rev().collect::<String>()).collect()
}


fn build_south_east_dir(content: &str) -> Vec<String> {
    let base: Vec<String> = build_east_dir(content);
    let length: usize = base.len();
    let width: usize = base.get(0).unwrap().len();

    let num_strs: usize = length+width-1;
    let mut result: Vec<String> = vec!["".to_string(); num_strs];

    // top-right traingle, include middle
    for col in 0..width {
        let val: usize = (width-1)-col;
        let row: usize = 0;
        let (mut nrow, mut ncol) = (row, col);
        while nrow < length && ncol < width{
            let char_at_pos: char = base.get(nrow).unwrap().chars().nth(ncol).unwrap();
            result.get_mut(val).unwrap().push(char_at_pos);
            
            nrow += 1;
            ncol += 1;
        }
    }

    // bottom-left triangle, excluding middle
    for row in 1..length {
        let val: usize = (width-1)+row;
        let col: usize = 0;
        let (mut nrow, mut ncol) = (row, col);
        while nrow < length{
            let char_at_pos: char = base.get(nrow).unwrap().chars().nth(ncol).unwrap();
            result.get_mut(val).unwrap().push(char_at_pos);
            nrow +=1;
            ncol +=1;
        }
    }

    result
}

fn build_north_west_dir(content: &str) -> Vec<String> {
    build_south_east_dir(content).iter().map(|x| x.chars().rev().collect::<String>()).collect()
}

fn build_south_west_dir(content: &str) -> Vec<String> {
    let base: Vec<String> = build_east_dir(content);
    let length: usize = base.len();
    let width: usize = base.get(0).unwrap().len();

    let num_strs: usize = length+width-1;
    let mut result: Vec<String> = vec!["".to_string(); num_strs];

    // top-left traingle, include middle
    for col in 0..width {
        let val: usize = col;
        let row: usize = 0;
        let (mut nrow, mut ncol) = (row, col);
        while nrow < length{
            let char_at_pos: char = base.get(nrow).unwrap().chars().nth(ncol).unwrap();
            result.get_mut(val).unwrap().push(char_at_pos);
            
            nrow += 1;
            if ncol == 0 {
                break
            } else {
                ncol -= 1;
            }
        }
    }

    // bottom-right triangle, excluding middle
    for row in 1..length {
        let val: usize = (width-1)+row;
        let col: usize = width-1;
        let (mut nrow, mut ncol) = (row, col);
        while nrow < length{
            let char_at_pos: char = base.get(nrow).unwrap().chars().nth(ncol).unwrap();
            result.get_mut(val).unwrap().push(char_at_pos);
            
            nrow +=1;
            ncol -=1;
        }
    }

    result
}

fn build_north_east_dir(content: &str) -> Vec<String> {
    build_south_west_dir(content).iter().map(|x| x.chars().rev().collect::<String>()).collect()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dirs() {
        let input: &str = "ABCD\nEFGH\nIJKL\nMNOP";

        let n_dir: Vec<String> = build_north_dir(input);
        assert_eq!(n_dir, vec!["AEIM","BFJN","CGKO","DHLP"]);
        let s_dir: Vec<String> = build_south_dir(input);
        assert_eq!(s_dir, vec!["MIEA","NJFB","OKGC","PLHD"]);

        let e_dir: Vec<String> = build_east_dir(input);
        assert_eq!(e_dir, vec!["ABCD","EFGH","IJKL","MNOP"]);
        let w_dir: Vec<String> = build_west_dir(input);
        assert_eq!(w_dir, vec!["DCBA","HGFE","LKJI","PONM"]);

        let se_dir: Vec<String> = build_south_east_dir(input);
        assert_eq!(se_dir, vec!["D", "CH", "BGL", "AFKP", "EJO", "IN", "M"]);
        let nw_dir: Vec<String> = build_north_west_dir(input);
        assert_eq!(nw_dir, vec!["D", "HC", "LGB", "PKFA", "OJE", "NI", "M"]);

        let sw_dir: Vec<String> = build_south_west_dir(input);
        assert_eq!(sw_dir, vec!["A", "BE", "CFI", "DGJM", "HKN", "LO", "P"]);
        let ne_dir: Vec<String> = build_north_east_dir(input);
        assert_eq!(ne_dir, vec!["A", "EB", "IFC", "MJGD", "NKH", "OL", "P"]);


        let input: &str = "ABC\nEFG\nIJK\nMNO";
        let se_dir: Vec<String> = build_south_east_dir(input);
        assert_eq!(se_dir, vec!["C", "BG", "AFK", "EJO", "IN", "M"]);
    }

}