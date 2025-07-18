use std::{collections::{HashMap, HashSet}, env, fs::read_to_string};
use priority_queue::PriorityQueue;


pub fn day_10_p1_soln() -> u32 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let (mut map, p_q) = parse_input(&raw);
    sum_of_trailheads(&mut map, &p_q)
}

fn sum_of_trailheads(map: &mut HashMap<Coord, Square>, p_q: &PriorityQueue<Coord, u8>) -> u32 {
    let mut trail_sum: u32 = 0;
    for (curr_coord, p) in p_q.clone().into_sorted_iter() {
        //println!("{:?} {} {}", (row,col), p, sum_sq(map.get(&(row,col)).unwrap()));
        update_square(map, &curr_coord);
        //println!("{:?} {} {}", (row,col), p, sum_sq(map.get(&(row,col)).unwrap()));
        if p == 0 {
            let sq: &Square = map.get(&curr_coord).unwrap();
            trail_sum += sq.summits.len() as u32;
        }
    }
    trail_sum
}


fn update_square(map: &mut HashMap<Coord, Square>, curr_coord: &Coord) {
    let mut reachable_summits: HashSet<Coord> = HashSet::new();
    let curr_num: u8 = map.get(&curr_coord).unwrap().num;
    let (row, col) = (curr_coord.r, curr_coord.c);

    if row != 0 {// prevent underflow
        if let Some(sq) = map.get(&coord(row-1,col)) {// north
            reachable_summits.extend(get_summits_if_smooth_ascent(sq, curr_num).iter());
        }
    }
    if let Some(sq) = map.get(&coord(row,col+1)) {// east
        reachable_summits.extend(get_summits_if_smooth_ascent(sq, curr_num).iter());
    }
    if let Some(sq) = map.get(&coord(row+1,col)) {// south
        reachable_summits.extend(get_summits_if_smooth_ascent(sq, curr_num).iter());
    }
    if col != 0 {// prevent underflow
        if let Some(sq) = map.get(&coord(row,col-1)) {// west
            reachable_summits.extend(get_summits_if_smooth_ascent(sq, curr_num).iter());
        }
    }
    let bob: &mut Square = map.get_mut(&curr_coord).unwrap();
    bob.summits = reachable_summits;
}


fn get_summits_if_smooth_ascent(sq: &Square, num: u8) -> HashSet<Coord> {
    if sq.num == num + 1 {
        sq.summits.clone()
    } else { HashSet::new() }
}


fn parse_input(content: &str) -> (HashMap<Coord, Square>, PriorityQueue<Coord, u8>) {
    let mut map: HashMap<Coord, Square> = HashMap::new();
    let mut p_q: PriorityQueue<Coord, u8> = PriorityQueue::new();

    for (row, line) in content.lines().enumerate() {
        for (col, raw_num) in line.chars().enumerate() {
            let coord: Coord = coord(row,col);
            let num: u8 = raw_num.to_digit(10).unwrap() as u8;
            map.insert(coord, new_s(num, coord));
            if num != 9 { p_q.push(coord, num); }
        }
    }

    (map, p_q)
}

#[derive(Clone, PartialEq, Eq)]
/// A Square holds the value of that square, and all of the 9's accessible from each direction
struct Square {
    pos: Coord,
    num: u8,
    summits: HashSet<Coord>
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    r: usize,
    c: usize,
}
fn coord(r: usize, c: usize) -> Coord {
    Coord{r:r, c:c}
}

fn new_s(num: u8, coord: Coord) -> Square {
    if num == 9 {
        Square{pos: coord, num: num, summits: HashSet::from([coord])}
    } else {
        Square{pos: coord, num: num, summits: HashSet::new()}
    }
    
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spec_example() {
        let input: &str = "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        let (mut map,p_q) = parse_input(&input);
        let ans: u32 = sum_of_trailheads(&mut map, &p_q);
        assert_eq!(ans, 36);
    }
}
