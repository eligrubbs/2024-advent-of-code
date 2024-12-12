use std::{env, fmt, fs::read_to_string, str::Chars};


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DiskBlock {
    // if is_free is true, id is Some, else None
    pub is_free: bool,
    pub id: Option<u64>,

}
impl fmt::Display for DiskBlock {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", if self.is_free {".".to_string()} else {self.id.unwrap().to_string()})
    }
}

fn full_disk(id: u64) -> DiskBlock {
    DiskBlock{is_free: false, id: Some(id)}
}
fn free_disk() -> DiskBlock {
    DiskBlock{is_free: true, id: None}
}

pub fn get_x_full_disks(num: u64, id: u64) -> Vec<DiskBlock> {
    let mut result: Vec<DiskBlock> = vec![];
    for _ in 0..num {
        result.push(full_disk(id));
    }
    result
}

pub fn get_x_free_disks(num: u64) -> Vec<DiskBlock> {
    let mut result: Vec<DiskBlock> = vec![];
    for _ in 0..num {
        result.push(free_disk());
    }
    result
}

pub fn parse_day_9_input() -> Vec<DiskBlock> {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    parse_input(&raw)
}


/// content is a string that only contains number chars 0-9.  
/// content is odd in length.  
/// 
/// Returns a string with id replicated for the number of blocks in that file and .'s in free space
fn parse_input(content: &str) -> Vec<DiskBlock> {
    assert!(content.len() % 2 != 0);

    let mut result: Vec<DiskBlock> = vec![];
    let mut id: u64 = 0;

    let mut iterator: Chars<'_> = content[0..content.len()-1].chars();
    while let (Some(char_1), Some(char_2)) = (iterator.next(), iterator.next()) {
        let block_len: u64 = char_1.to_digit(10).unwrap() as u64;
        let free_space: u64 = char_2.to_digit(10).unwrap() as u64;

        // insert `block_len` blocks with id `id` into result
        result.extend(get_x_full_disks(block_len, id));

        result.extend(get_x_free_disks(free_space));

        id += 1;
    }

    // push last file that only has last char associated with it
    let final_val: u64 = content.chars().last().unwrap().to_digit(10).unwrap() as u64;
    result.extend(get_x_full_disks(final_val, id));

    assert_eq!(content.chars().map(|ch| ch.to_digit(10).unwrap()).sum::<u32>() as u64, result.len() as u64);
    result
}


#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse() {
        let input: &str = "233313312141413140204";
        let mut result: Vec<DiskBlock> = vec![];
        result.extend(get_x_full_disks(2, 0));
        result.extend(get_x_free_disks(3));

        result.extend(get_x_full_disks(3, 1));
        result.extend(get_x_free_disks(3));

        result.extend(get_x_full_disks(1, 2));
        result.extend(get_x_free_disks(3));

        result.extend(get_x_full_disks(3, 3));
        result.extend(get_x_free_disks(1));

        result.extend(get_x_full_disks(2, 4));
        result.extend(get_x_free_disks(1));

        result.extend(get_x_full_disks(4, 5));
        result.extend(get_x_free_disks(1));

        result.extend(get_x_full_disks(4, 6));
        result.extend(get_x_free_disks(1));

        result.extend(get_x_full_disks(3, 7));
        result.extend(get_x_free_disks(1));
        result.extend(get_x_full_disks(4, 8));
        result.extend(get_x_full_disks(2, 9));
        result.extend(get_x_full_disks(4, 10));

        assert_eq!(result, parse_input(input))
    }
}