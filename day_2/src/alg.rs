
fn entry_is_safe(entry: &Vec<i32>) -> bool {
    let differences: Vec<i32> = entry.windows(2).map(|s: &[i32]| s[1] - s[0]).collect();

    let first = differences.get(0).unwrap().clone();

    let all_elems_have_same_sign: bool = differences.iter().all(|&s| ((first > 0) && (s > 0)) ||
                                                                          ((first < 0) && (s < 0)) );

    let all_elems_gradual: bool = differences.into_iter()
        .map(|x| x.abs())
        .all(|x| (x <= 3) && (x >= 1));

    all_elems_have_same_sign && all_elems_gradual
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
}