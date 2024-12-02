use std::collections::{HashMap, HashSet};

use crate::parser::read_input_file;

fn list_distance_calculator(mut list_1: Vec<i32>, mut list_2: Vec<i32>) -> i32 {
    list_1.sort();
    list_2.sort();
    list_1.into_iter().zip(list_2).map(|(a,b)| (a-b).abs()).sum()
}

pub fn day_1_p1_soln() -> i32 {
    let (list_1, list_2) = read_input_file();
    list_distance_calculator(list_1, list_2)
}

pub fn day_1_p2_soln() -> i32 {
    /*
    To meet the requirements, the following steps were followed:
    1. HashMap list_1 - we only need to know members of this list
    2. Create empty map to store list_2 (item, frequency count)
    3. Filter list_2 to populate the hashmap with proper values
    4. iterate over hashmap and accumualte similarity score
    
     */
    let (list_1, list_2) = read_input_file();

    let set_list_1: HashSet<i32> = list_1.clone().into_iter().collect();
    let mut list_2_counts: HashMap<i32, i32> = HashMap::new();

    list_2.iter()
          .filter(|&x| set_list_1.contains(x))
          .for_each(|x| {
                        list_2_counts.entry(*x)
                                     .and_modify(|cnt| {*cnt += 1})
                                     .or_insert(1);
                    });

    list_2_counts.iter().fold(0, |mut acc, (key, val)| 
                        {
                            acc += key * val;
                            acc
                        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_calc() {
        let list_1: Vec<i32> = vec![12, 10, 14];
        let list_2: Vec<i32> = vec![10, 3, 10];
        assert_eq!(13, list_distance_calculator(list_1, list_2))
    }
}