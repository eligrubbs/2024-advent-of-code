use std::{env, fmt, fs::read_to_string, str::Chars};

pub fn day_9_p1_soln() -> u64 {
    let input: Vec<DiskBlock> = parse_day_9_input();
    calc_check_sum(&input)
}

/// take content with dots scattered throughout and calcualte the checksum
fn calc_check_sum(contents: &Vec<DiskBlock>) -> u64 {
    let mut check_sum: u64 = 0;

    let mut content: Vec<DiskBlock> = contents.clone();
    println!("Reversing");
    rearrange_empty_blocks(&mut content);
    println!("Calculating checksum");

    let mut iterator = content.iter().enumerate();

    while let Some((ind, block)) = iterator.next() {
        if block.is_free { continue }
        check_sum += block.id.unwrap() * (ind as u64);
    }
    check_sum

}

fn rearrange_empty_blocks(blocks: &mut Vec<DiskBlock>) {
    let mut dot_ptr: usize = ind_first_empty(blocks);
    let mut end_ptr: usize = ind_last_full(blocks);

    while dot_ptr < end_ptr {
        //swap pointers
        blocks.swap(dot_ptr, end_ptr);
        // increment ptrs
        dot_ptr = ind_first_empty(blocks);
        end_ptr = ind_last_full(blocks);
    }

}

fn ind_first_empty(blocks: &Vec<DiskBlock>) -> usize {
    blocks.iter()
          .enumerate()
          .filter(|(_, &block)| block.is_free)
          .map(|(ind, _)| ind)
          .min().unwrap()
}

fn ind_last_full(blocks: &Vec<DiskBlock>) -> usize {
    let first_ind: usize = blocks.iter()
    .rev()
    .enumerate()
    .filter(|(_, &block)| !block.is_free && !block.tried_to_move)
    .map(|(ind, _)| ind)
    .min().unwrap();

    (blocks.len()-1) - first_ind
}

////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct DiskBlock {
    // if is_free is true, id is Some, else None
    is_free: bool,
    id: Option<u64>,
    // part 2
    group_size: usize,
    tried_to_move: bool,

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

fn full_disk(id: u64, group_size: usize) -> DiskBlock {
    DiskBlock{is_free: false, id: Some(id), group_size, tried_to_move: false}
}
fn free_disk(group_size: usize) -> DiskBlock {
    DiskBlock{is_free: true, id: None, group_size, tried_to_move: false}
}

fn get_x_full_disks(num: usize, id: u64) -> Vec<DiskBlock> {
    let mut result: Vec<DiskBlock> = vec![];
    for _ in 0..num {
        result.push(full_disk(id, num));
    }
    result
}

fn get_x_free_disks(num: usize) -> Vec<DiskBlock> {
    let mut result: Vec<DiskBlock> = vec![];
    for _ in 0..num {
        result.push(free_disk(num));
    }
    result
}

fn parse_day_9_input() -> Vec<DiskBlock> {
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
        let block_len: usize = char_1.to_digit(10).unwrap() as usize;
        let free_space: usize = char_2.to_digit(10).unwrap() as usize;

        // insert `block_len` blocks with id `id` into result
        result.extend(get_x_full_disks(block_len, id));

        result.extend(get_x_free_disks(free_space));

        id += 1;
    }

    // push last file that only has last char associated with it
    let final_val: usize = content.chars().last().unwrap().to_digit(10).unwrap() as usize;
    result.extend(get_x_full_disks(final_val, id));

    assert_eq!(content.chars().map(|ch| ch.to_digit(10).unwrap()).sum::<u32>() as u64, result.len() as u64);
    result
}
