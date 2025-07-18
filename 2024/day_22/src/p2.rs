use std::collections::HashSet;
use std::{collections::HashMap, env};
use std::fs::read_to_string;
use crate::p1::{parse_input, next_secret};


pub fn day_22_p2_soln() -> u64 {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    let secs: Vec<u64> = parse_input(&raw);

    let mut seq_map: HashMap<(i8,i8,i8,i8), u64> = HashMap::new();

    all_buyer_seqs(&secs, &mut seq_map);

    seq_map.values().max().unwrap().clone()
}

pub fn all_buyer_seqs(secs: &Vec<u64>, map: &mut HashMap<(i8,i8,i8,i8), u64>) {
    for &secret in secs {
        seqs_for_a_buyer(map, secret);
    }
}


fn seqs_for_a_buyer(map: &mut HashMap<(i8,i8,i8,i8), u64>, secret: u64) -> u64 {
    let mut secret: u64 = secret;
    let mut seq: Vec<i8> = vec![];
    let mut seq_seen: HashSet<(i8,i8,i8,i8)> = HashSet::new();
    // init sequence
    for _ in 0..4 {
        let new_secret: u64 = next_secret(secret);
        seq.push( price_from(new_secret) - price_from(secret) );
        secret = new_secret;
    }
    seq_seen.insert(to_tup(&seq));

    for _ in 0..1996 {
        if !seq_seen.contains(&to_tup(&seq)) {
            map.entry(to_tup(&seq))
            .and_modify(|e| {*e += price_from(secret) as u64 ;})
            .or_insert(price_from(secret) as u64);

            seq_seen.insert(to_tup(&seq));
        }

        let new_secret: u64 = next_secret(secret);
        seq.push( price_from(new_secret) - price_from(secret) );
        seq.remove(0);
        secret = new_secret;
    }

    secret
}

pub fn to_tup(v: &Vec<i8>) -> (i8,i8,i8,i8) {
    assert_eq!(4, v.len());
    (v[0],v[1],v[2],v[3])
}

pub fn price_from(n: u64) -> i8 {
    (n % 10) as i8
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spec() {
        let secs: Vec<u64> = vec![1,2,3,2024];

    let mut seq_map: HashMap<(i8,i8,i8,i8), u64> = HashMap::new();

    all_buyer_seqs(&secs, &mut seq_map);

    println!("{}", seq_map.get(&(-2,1,-1,3)).unwrap());

    assert_eq!(23, seq_map.values().max().unwrap().clone())
    }
}