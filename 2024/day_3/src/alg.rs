use crate::parser::read_input_file;


fn calc_product_sum(list: &Vec<(i32, i32)>) -> i32{
    list.iter().map(|(l, r)| l * r).sum::<i32>()
}

pub fn day_3_p1_soln() -> i32 {
    let result = read_input_file(true);
    calc_product_sum(&result)
}

pub fn day_3_p2_soln() -> i32 {
    let pairs: Vec<(i32, i32)> = read_input_file(false);
    calc_product_sum(&pairs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_producter(){
        let input: Vec<(i32, i32)> = vec![(2,4),(5,5),(11,8),(8,5)];
        assert_eq!(161, calc_product_sum(&input));
    }
}