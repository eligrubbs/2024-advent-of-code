use std::iter::zip;
use crate::parser::parse_day_7_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
    Ext, // to add after Add/Mul work for p1
}

fn ext(left: i64, right: i64) -> i64 {
    let digits_to_shift: u32 = right.checked_ilog10().unwrap_or(0) + 1;
    let base: i64 = 10;
    let shifted_left: i64 = left * base.pow(digits_to_shift);
    right + shifted_left
}

fn greater_than_after_add(left: i64, right: i64, total: i64) -> bool {
    if left + right > total { true } else { false }
}

fn greater_than_after_mul(left: i64, right: i64, total: i64) -> bool {
    if left * right > total { true } else { false }
}

fn greater_than_after_ext(left: i64, right: i64, total: i64) -> bool {
    if ext(left, right) > total { true } else { false }
}

fn greater_than_after(oper: Operation, left: i64, right: i64, total: i64) -> bool {
   use Operation::*;
   match oper {
    Add => { greater_than_after_add(left, right, total) }
    Mul => { greater_than_after_mul(left, right, total) }
    Ext => { greater_than_after_ext(left, right, total) }
   }
}

fn comb(oper: Operation, left: i64, right: i64) -> i64 {
    use Operation::*;
    match oper {
        Add => {left + right},
        Mul => {left * right},
        Ext => {ext(left, right)}
    }
}

fn get_all_operations_possible(line: &Vec<i64>, p2: bool) -> Vec<Vec<Operation>> {
    use Operation::*;
    let mut all_operations: Vec<Vec<Operation>> = vec![vec![Add; line.len() - 1]];
    for i in 0..(line.len()-1) {
        let mut ops_added: Vec<Vec<Operation>> = vec![];
        for operation in &all_operations {
            let mut new_op: Vec<Operation> = operation.clone();
            // multiplication
            new_op[i] = Mul;
            ops_added.push(new_op.clone());
            if p2 {
                // extension
                new_op[i] = Ext;
                ops_added.push(new_op.clone());
            }
        }
        all_operations.extend(ops_added);
    }
    let mut base: usize = 2;
    if p2 {
        base= 3;
    }

    assert_eq!(all_operations.len(), base.pow(u32::try_from(line.len()-1).unwrap()));
    
    all_operations
}

/// See if a line can equal the total after applying additions or multiplications
/// 
/// March down operations and return index of operation that made left go over total
/// Swap a parent if greater_than_op is true\
/// 
/// Returns:
/// None: None if the line equals total with the operations
/// usize: the index of the operation that makes combo go over total, or operations.len() if they are less than total
fn operations_equal_total(total: i64, line: &Vec<i64>, operations: &Vec<Operation>) -> Option<usize> {
    // true => add, false => mult
    assert!(line.len() - 1 == operations.len());
    let mut left: i64 = line[0];
    for (i, is_add) in zip(1..line.len(), operations) {
        let right = line[i];
        if greater_than_after(*is_add, left, right, total) {
            return Some(i-1);
        } else {
            left = comb(*is_add, left, right);
        }

    }
    // Chance that left is below total
    if left < total {
        return Some(operations.len());
    }
    // If here, then total should equal left, else something very bad happenend
    assert_eq!(total, left);
    None
}

/// Determines if there exist operations to get elems in `line` to equal `total`
/// 
/// Returns the list of operations (true => add, false => multiply) that make it so.
/// Returns None if it is not possible.
fn line_can_match_total(line: &Vec<i64>, total: i64, p2: bool) -> Option<Vec<Operation>> {
    let all_operations: Vec<Vec<Operation>> = get_all_operations_possible(line, p2);
    for operations in &all_operations {
        let none_if_equal: Option<usize> = operations_equal_total(total, line, operations);
        if none_if_equal.is_none() {
            return Some(operations.clone());
        }
    }
    None
}



pub fn day_7_p1_soln() -> i64 {
    let comb_lines: Vec<(i64, Vec<i64>)> = parse_day_7_input();
    let mut valid_lines_inds: Vec<usize> = vec![];
    let mut valid_operations: Vec<Vec<Operation>> = vec![];
    let mut comb_score = 0;
    for (i, (total, line)) in comb_lines.iter().enumerate() {
        if let Some(bob) = line_can_match_total(line, *total, false) {
            valid_operations.push(bob);
            valid_lines_inds.push(i);
            comb_score += total;
        }
    }

    comb_score
}

pub fn day_7_p2_soln() -> i64 {
    let comb_lines: Vec<(i64, Vec<i64>)> = parse_day_7_input();
    let mut valid_lines_inds: Vec<usize> = vec![];
    let mut valid_operations: Vec<Vec<Operation>> = vec![];
    let mut comb_score = 0;
    for (i, (total, line)) in comb_lines.iter().enumerate() {
        if let Some(bob) = line_can_match_total(line, *total, true) {
            valid_operations.push(bob);
            valid_lines_inds.push(i);
            comb_score += total;
        }
    }

    comb_score
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_operation() {
        assert_eq!(156, ext(15, 6));
        assert_eq!(150, ext(15, 0));
        assert_eq!(6435, ext(6, 435));
    }
}