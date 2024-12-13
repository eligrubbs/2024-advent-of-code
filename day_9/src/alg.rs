use itertools::Itertools; // uncomment import for print debugging
use crate::parser::{parse_day_9_input, DiskBlock, get_x_free_disks, get_x_full_disks};


pub fn day_9_p1_soln() -> u64 {
    let input: Vec<DiskBlock> = parse_day_9_input();
    calc_check_sum(&input)
}

pub fn day_9_p2_soln() -> u64 {
    let input: Vec<DiskBlock> = parse_day_9_input();
    calc_check_sum_p2(&input)
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

fn calc_check_sum_p2(contents: &Vec<DiskBlock>) -> u64 {
    let mut check_sum: u64 = 0;

    let mut content: Vec<DiskBlock> = contents.clone();
    println!("Reversing");
    rearrange_whole_empty_blocks(&mut content);
    println!("Calculating checksum");

    let mut iterator = content.iter().enumerate();

    while let Some((ind, block)) = iterator.next() {
        if block.is_free { continue }
        check_sum += block.id.unwrap() * (ind as u64);
    }
    check_sum

}

fn rearrange_whole_empty_blocks(blocks: &mut Vec<DiskBlock>) {
    let mut dot_ptr: usize = ind_first_empty(blocks);
    let mut end_ptr: usize = ind_last_full(blocks);

    while blocks[end_ptr].id.unwrap() > 0 && dot_ptr < end_ptr {
        let (dot_g_sz, end_g_sz) = (blocks[dot_ptr].group_size, blocks[end_ptr].group_size);
        
        let mut num_swapped: usize = 0;
        'swapper: for dot_ind in dot_ptr..(end_ptr-end_g_sz+1) {
            if num_swapped >= end_g_sz{ // I've swapped current end_group
                break 'swapper;
            }
            //check if whole block @ end_ptr will fit
            if blocks[dot_ind].group_size < end_g_sz {
                continue;
            } else { // swap these guys
                // update group size to be current - group
                if blocks[dot_ind].group_size != end_g_sz{//split current dot groups into two groups anticipating swaps
                    let diff: usize = blocks[dot_ind].group_size - end_g_sz;
                    for temp in dot_ind..(dot_ind +1+diff ) {
                        blocks.get_mut(temp).unwrap().group_size = end_g_sz;
                    }
                    for temp in (dot_ind+1+diff)..(dot_ind+dot_g_sz) {
                        blocks.get_mut(temp).unwrap().group_size = diff;
                    }
                    
                }
                // say I've swapped it
                blocks.get_mut(end_ptr-num_swapped).unwrap().tried_to_move = true;
                // perform swap
                blocks.swap(dot_ind, end_ptr-num_swapped);
                num_swapped += 1;
            }
        }
        if num_swapped == 0 {
            for temp in 0..end_g_sz {
                blocks.get_mut(end_ptr-temp).unwrap().tried_to_move = true;
            }
        }
        // increment ptrs
        dot_ptr = ind_first_empty(blocks);
        end_ptr = ind_last_full(blocks);
    }

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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rearrange() {
        let mut input: Vec<DiskBlock> = vec![];
        input.extend(get_x_full_disks(2, 0));
        input.extend(get_x_free_disks(3));

        input.extend(get_x_full_disks(3, 1));
        input.extend(get_x_free_disks(3));

        input.extend(get_x_full_disks(1, 2));
        input.extend(get_x_free_disks(3));

        input.extend(get_x_full_disks(3, 3));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(2, 4));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(4, 5));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(4, 6));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(3, 7));
        input.extend(get_x_free_disks(1));
        input.extend(get_x_full_disks(4, 8));
        input.extend(get_x_full_disks(2, 9));

        let mut result: Vec<DiskBlock> = vec![];
        result.extend(get_x_full_disks(2, 0));
        result.extend(get_x_full_disks(2, 9));
        result.extend(get_x_full_disks(1, 8));
        result.extend(get_x_full_disks(3, 1));
        result.extend(get_x_full_disks(3, 8));
        result.extend(get_x_full_disks(1, 2));
        result.extend(get_x_full_disks(3, 7));
        result.extend(get_x_full_disks(3, 3));
        result.extend(get_x_full_disks(1, 6));
        result.extend(get_x_full_disks(2, 4));
        result.extend(get_x_full_disks(1, 6));
        result.extend(get_x_full_disks(4, 5));
        result.extend(get_x_full_disks(2, 6));
        
        result.extend(get_x_free_disks(14));

        rearrange_empty_blocks(&mut input);
        // uncomment below print statements for debugging
        //println!("{}", input.iter().format(""));
        //println!("{}", result.iter().format(""));
        assert_eq!(input, result);
    }

    #[test]
    fn test_checksum() {
        let mut input: Vec<DiskBlock> = vec![];
        input.extend(get_x_full_disks(2, 0));
        input.extend(get_x_free_disks(3));

        input.extend(get_x_full_disks(3, 1));
        input.extend(get_x_free_disks(3));

        input.extend(get_x_full_disks(1, 2));
        input.extend(get_x_free_disks(3));

        input.extend(get_x_full_disks(3, 3));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(2, 4));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(4, 5));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(4, 6));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(3, 7));
        input.extend(get_x_free_disks(1));
        input.extend(get_x_full_disks(4, 8));
        input.extend(get_x_full_disks(2, 9));
        let checksum: u64 = calc_check_sum(&input);
        assert_eq!(checksum, 1928);
    }

    // p2 tests

    #[test]
    fn test_rearrange_p2() {
        let mut input: Vec<DiskBlock> = vec![];
        input.extend(get_x_full_disks(2, 0));
        input.extend(get_x_free_disks(3));

        input.extend(get_x_full_disks(3, 1));
        input.extend(get_x_free_disks(3));

        input.extend(get_x_full_disks(1, 2));
        input.extend(get_x_free_disks(3));

        input.extend(get_x_full_disks(3, 3));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(2, 4));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(4, 5));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(4, 6));
        input.extend(get_x_free_disks(1));

        input.extend(get_x_full_disks(3, 7));
        input.extend(get_x_free_disks(1));
        input.extend(get_x_full_disks(4, 8));
        input.extend(get_x_full_disks(2, 9));

        let mut result: Vec<DiskBlock> = vec![];
        result.extend(get_x_full_disks(2, 0));
        result.extend(get_x_full_disks(2, 9));
        result.extend(get_x_full_disks(1, 8));
        result.extend(get_x_full_disks(3, 1));
        result.extend(get_x_full_disks(3, 8));
        result.extend(get_x_full_disks(1, 2));
        result.extend(get_x_full_disks(3, 7));
        result.extend(get_x_full_disks(3, 3));
        result.extend(get_x_full_disks(1, 6));
        result.extend(get_x_full_disks(2, 4));
        result.extend(get_x_full_disks(1, 6));
        result.extend(get_x_full_disks(4, 5));
        result.extend(get_x_full_disks(2, 6));
        
        result.extend(get_x_free_disks(14));

        rearrange_whole_empty_blocks(&mut input);
        // uncomment below print statements for debugging
        println!("{}", input.iter().format(""));
        println!("{}", result.iter().format(""));
        assert_eq!(input, result);
    }
}