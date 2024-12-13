use std::{str::Chars, env, fs::read_to_string};


pub fn day_9_p2_soln() -> u64 {
    let mut input: Vec<DiskBlock> = parse_day_9_input();
    swap_blocks(&mut input);
    calc_checksum(&input)
}

fn calc_checksum(input: &Vec<DiskBlock>) -> u64 {
    let mut checksum: u64 = 0;
    for block in input.iter() {
        if !block.is_free {
            for ind in block.pos..(block.pos+block.size) {
                println!("{} {} {} {}", block.id.unwrap(), ind, block.size, block.tried_to_swap);
                checksum += (ind as u64) * block.id.unwrap();
            }
        }
    }

    checksum
}

fn swap_blocks(input: &mut Vec<DiskBlock>) {
    let mut end_ptr: usize = get_last_block(input);
    let mut dot_ptr: usize = get_first_dot(input);

    while dot_ptr < end_ptr {
        'search_for_swap: for free_ind in dot_ptr..input.len() {
            let free_block: &DiskBlock = input.get(free_ind).unwrap();
            if !free_block.is_free { 
                continue;
            }
            let (dot_size, end_size) = (input[free_ind].size, input[end_ptr].size);
            if dot_size >= end_size { // can swap these!
                let diff: usize = dot_size - end_size;

                input.get_mut(end_ptr).unwrap().tried_to_swap = true;
                //
                let temp_pos = input.get(end_ptr).unwrap().pos;
                input.get_mut(end_ptr).unwrap().pos = input.get(free_ind).unwrap().pos;
                input.get_mut(free_ind).unwrap().pos = temp_pos;
                //
                input.swap(free_ind, end_ptr);
                //if there is a remainder of spaces, insert empty space after
                if diff > 0 {
                    input.insert(free_ind+1, get_free_disk(free_ind+end_size+1, diff));
                }

                break 'search_for_swap;
            }
        }
        // I am here if I swapped or not, check if I swapped
        if !input.get(end_ptr).unwrap().is_free { // I didn't swap
            input.get_mut(end_ptr).unwrap().tried_to_swap = true;
        }
        // move on to next dudes
        dot_ptr = get_first_dot(input);
        end_ptr = get_last_block(input);
    }

    
}

fn get_first_dot(input: &Vec<DiskBlock>) -> usize {
    input.iter().enumerate()
                .filter(|(_, &val)| val.is_free)
                .map(|(ind, _)| ind)
                .min().unwrap()
}

fn get_last_block(input: &Vec<DiskBlock>) -> usize {
    input.iter().enumerate()
                .filter(|(_, &val)| !val.is_free && !val.tried_to_swap)
                .map(|(ind, _)| ind)
                .max().unwrap()
}

///////// 

fn parse_day_9_input() -> Vec<DiskBlock> {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    parse_input(&raw)
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct DiskBlock {
    is_free: bool,
    id: Option<u64>,
    // pos and size are on same scale
    pos: usize, // position of first elem in hypothetical expanded setting
    size: usize, // number of blocks in this disk block
    tried_to_swap: bool,
}

/// content is a string that only contains number chars 0-9.  
/// content is odd in length.  
/// 
/// Returns a string with id replicated for the number of blocks in that file and .'s in free space
fn parse_input(content: &str) -> Vec<DiskBlock> {
    assert!(content.len() % 2 != 0);

    let mut result: Vec<DiskBlock> = vec![];
    let mut id: u64 = 0;
    let mut curr_pos: usize = 0;

    let mut iterator: Chars<'_> = content[0..content.len()-1].chars();
    while let (Some(char_1), Some(char_2)) = (iterator.next(), iterator.next()) {
        let block_len: usize = char_1.to_digit(10).unwrap() as usize;
        let free_space: usize = char_2.to_digit(10).unwrap() as usize;

        // insert `block_len` blocks with id `id` into result
        result.push(get_full_disk(curr_pos, block_len, id));
        curr_pos += block_len;
        result.push(get_free_disk(curr_pos, free_space));
        curr_pos += free_space;

        id += 1;
    }

    // push last file that only has last char associated with it
    let final_val: usize = content.chars().last().unwrap().to_digit(10).unwrap() as usize;
    result.push(get_full_disk(curr_pos, final_val, id));

    assert_eq!(content.len(), result.len());
    result
}

fn get_full_disk(pos: usize, group_size: usize, id: u64) -> DiskBlock {
    DiskBlock{is_free: false, size: group_size, pos: pos, id: Some(id), tried_to_swap: false}
}
fn get_free_disk(pos: usize, group_size: usize) -> DiskBlock {
    DiskBlock{is_free: true, size: group_size, pos: pos, id: None, tried_to_swap: false}
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_spec_example() {
        let raw_input: &str = "2333133121414131402";
        let mut input: Vec<DiskBlock> = parse_input(&raw_input);
        for block in &input {
            if !block.is_free {
                println!("{} {} {} {}", block.id.unwrap(), block.pos, block.size, block.tried_to_swap);
            }
        }
        swap_blocks(&mut input);
        let check = calc_checksum(&input);

        assert_eq!(check, 2858);
    }
}