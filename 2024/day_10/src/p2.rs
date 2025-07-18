use std::{collections::HashMap, env, fs::read_to_string};
use priority_queue::PriorityQueue;

pub fn day_10_p2_soln() -> u32 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let (mut map, p_q) = parse_input(&raw);
    sum_of_trailheads(&mut map, &p_q)
}

fn sum_of_trailheads(map: &mut HashMap<(usize, usize), Square>, p_q: &PriorityQueue<(usize,usize), u8>) -> u32 {
    let mut trail_sum: u32 = 0;
    for ((row, col), p) in p_q.clone().into_sorted_iter() {
        // println!("{:?} {}", (row,col), p, );
        update_square(map, (row,col));
        // println!("{:?} {}", (row,col), p);
        if p == 0 {
            let sq: &Square = map.get(&(row,col)).unwrap();
            trail_sum += sum_sq(sq);
        }
    }
    trail_sum
}

fn sum_sq(sq: &Square) -> u32 {
    sq.n_c + sq.e_c + sq.s_c + sq.w_c
}

fn update_square(map: &mut HashMap<(usize, usize), Square>, (row,col): (usize, usize)) {
    let mut n_n: u32 = 0;
    let mut e_n: u32 = 0;
    let mut s_n: u32 = 0;
    let mut w_n: u32 = 0;
    let curr_num = map.get(&(row,col)).unwrap().num;
    if row != 0 {// prevent underflow
        if let Some(sq) = map.get(&(row-1,col)) {// north
            n_n = val_based_on_square(sq, curr_num);
        }
    }
    if let Some(sq) = map.get(&(row,col+1)) {// east
        e_n = val_based_on_square(sq, curr_num);
    }
    if let Some(sq) = map.get(&(row+1,col)) {// south
        s_n = val_based_on_square(sq, curr_num);
    }
    if col != 0 {// prevent underflow
        if let Some(sq) = map.get(&(row,col-1)) {// west
            w_n = val_based_on_square(sq, curr_num);
        }
    }
    let bob: &mut Square = map.get_mut(&(row,col)).unwrap();
    bob.n_c = n_n;
    bob.e_c = e_n;
    bob.s_c = s_n;
    bob.w_c = w_n;
}


fn val_based_on_square(sq: &Square, num: u8) -> u32 {
    if sq.num == num + 1 {
        if sq.num == 9 {
            return 1
        } else {
            sq.n_c + sq.e_c + sq.s_c + sq.w_c
        }
    } else { 0 }   
}


fn parse_input(content: &str) -> (HashMap<(usize,usize), Square>, PriorityQueue<(usize, usize), u8>) {
    let mut result: HashMap<(usize, usize), Square> = HashMap::new();
    let mut p_q: PriorityQueue<(usize,usize), u8> = PriorityQueue::new();

    for (row, line) in content.lines().enumerate() {
        for (col, raw_num) in line.chars().enumerate() {
            let num: u8 = raw_num.to_digit(10).unwrap() as u8;
            result.insert((row,col), new_s(num, (row,col)));
            if num != 9 { p_q.push((row,col), num); }
        }
    }

    (result, p_q)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// A Square holds the value of that square, and all of the 9's accessible from each direction
struct Square {
    pos: (usize, usize),
    num: u8,
    n_c: u32,
    e_c: u32,
    w_c: u32,
    s_c: u32
}
fn new_s(num: u8, pos: (usize, usize)) -> Square {
    Square{pos, num: num, n_c:0, e_c: 0, w_c: 0, s_c:0}
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spec_example_p2() {
        let input: &str = "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        let (mut map,p_q) = parse_input(&input);
        let ans: u32 = sum_of_trailheads(&mut map, &p_q);
        assert_eq!(ans, 81);
    }
}
