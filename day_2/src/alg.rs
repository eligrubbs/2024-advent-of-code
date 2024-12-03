use std::collections::HashSet;
use crate::parser::read_input_file;

fn entry_is_safe(entry: &Vec<i32>) -> bool {
    let differences: HashSet<i32> = entry.windows(2).map(|s: &[i32]| s[1] - s[0]).collect();

    differences.is_subset(&HashSet::<i32>::from([1,2,3])) || 
    differences.is_subset(&HashSet::<i32>::from([-1,-2,-3]))

}

fn damp_entry_is_safe(entry: &Vec<i32>) -> bool {

    for i in 0..entry.len() {
        let mut missing_one_vec: Vec<i32> = vec![];
        for j in 0..entry.len() {
            if j != i {
                missing_one_vec.push(*entry.get(j).unwrap());
            }
        }

        if entry_is_safe(&missing_one_vec) {
            return true;
        }
    }

    false
}

pub fn day_2_p1_soln() -> i32 {
    let inputs: Vec<Vec<i32>> = read_input_file();
    inputs.into_iter().filter(|x| entry_is_safe(x)).count().try_into().unwrap()
}

pub fn day_2_p2_soln() -> i32 {
    let inputs: Vec<Vec<i32>> = read_input_file();
    inputs.into_iter().filter(|x| damp_entry_is_safe(x)).count().try_into().unwrap()
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_entry_is_safe() {
        let entries: Vec<Vec<i32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]];
        let answers: Vec<bool> = vec![true, false, false, false, false, true];

        assert_eq!(entry_is_safe(entries.get(0).unwrap()), answers[0]);
        assert_eq!(entry_is_safe(entries.get(1).unwrap()), answers[1]);
        assert_eq!(entry_is_safe(entries.get(2).unwrap()), answers[2]);
        assert_eq!(entry_is_safe(entries.get(3).unwrap()), answers[3]);
        assert_eq!(entry_is_safe(entries.get(4).unwrap()), answers[4]);
        assert_eq!(entry_is_safe(entries.get(5).unwrap()), answers[5]);

    }

    #[test]
    fn test_dampened_entry_is_safe() {
        let entries: Vec<Vec<i32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9]];
        let answers: Vec<bool> = vec![true, false, false, true, true, true];

        assert_eq!(damp_entry_is_safe(entries.get(0).unwrap()), answers[0]);
        assert_eq!(damp_entry_is_safe(entries.get(1).unwrap()), answers[1]);
        assert_eq!(damp_entry_is_safe(entries.get(2).unwrap()), answers[2]);
        assert_eq!(damp_entry_is_safe(entries.get(3).unwrap()), answers[3]);
        assert_eq!(damp_entry_is_safe(entries.get(4).unwrap()), answers[4]);
        assert_eq!(damp_entry_is_safe(entries.get(5).unwrap()), answers[5]);
    }
}