use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

use crate::network::Connection;


pub fn day_23_p1_soln() -> usize {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("input.txt");
    let raw: String = read_to_string(path.to_str().unwrap()).unwrap();
    
    let (network, computers) = parse_input(&raw);

    trip_conns_w_t(&network, &computers).len()
}

pub fn parse_input(content: &str) -> (HashSet<Connection>, HashSet<String>){
    let mut computers: HashSet<String> = HashSet::new();

    (content.lines().map(|line| {
        let mut bob: std::str::Split<'_, char> = line.split('-');
        let (comp1, comp2) = (bob.next().unwrap().to_string(), bob.next().unwrap().to_string());
        computers.insert(comp1.clone()); computers.insert(comp2.clone());
        Connection::new(comp1, comp2)
    }).collect(), computers)
}



fn trip_conns_w_t(network: &HashSet<Connection>, comps: &HashSet<String>) -> HashSet<String>{
    let mut three_way: HashSet<String> = HashSet::new();

    for comp in comps {
        for comp_conn in connections_to(&comp, network) {
            let second_comp: String = comp_conn.other(&comp).unwrap();

            for second_conn in connections_to(&second_comp, network) {
                let third_comp: String = second_conn.other(&second_comp).unwrap();
                let potential_3_way: String = make_triplet(&comp, &second_comp, &third_comp);
                let linking_conn: Connection = Connection::new(comp.clone(), third_comp.clone());

                if third_comp != *comp && network.contains(&linking_conn) && 
                   triplet_has_start_w_t(&potential_3_way) {
                    three_way.insert(potential_3_way);
                }
            }
        }
    }
    three_way
}


fn connections_to<'a> (comp: &String, network: &'a HashSet<Connection>) -> HashSet<&'a Connection> {
    network.iter().filter(|&conn| conn.in_conn(comp)).collect()
}


/// creates a network triplet where a,b,c are put in a comma seperated list in order
fn make_triplet(a: &String, b: &String, c: &String) -> String {
    min(min(a,b), c).to_owned() + "," + // smallest
    min(min(max(a, b), max(b,c)), max(a,c)) + "," + // 2nd largest
    max(max(a,b),c) // largest
}

fn triplet_has_start_w_t(trip: &String) -> bool {
    trip.chars().nth(0).unwrap() == 't' || trip.chars().nth(3).unwrap() == 't' || trip.chars().nth(6).unwrap() == 't'
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_triplet() {
        let a: String = "a".to_string();
        let b: String = "b".to_string();
        let c: String = "c".to_string();

        assert_eq!("a,b,c", make_triplet(&a, &b, &c));
        assert_eq!("a,b,c", make_triplet(&a, &c, &b));
        assert_eq!("a,b,c", make_triplet(&b, &c, &a));
        assert_eq!("a,b,c", make_triplet(&b, &a, &c));
        assert_eq!("a,b,c", make_triplet(&c, &a, &b));
        assert_eq!("a,b,c", make_triplet(&c, &b, &a));
    }

    #[test]
    fn test_spec() {
        let input: &str = "kh-tc\nqp-kh\nde-cg\nka-co\nyn-aq\nqp-ub
cg-tb\nvc-aq\ntb-ka\nwh-tc\nyn-cg\nkh-ub\nta-co\nde-co
tc-td\ntb-wq\nwh-td\nta-ka\ntd-qp\naq-cg\nwq-ub\nub-vc
de-ta\nwq-aq\nwq-vc\nwh-yn\nka-de\nkh-ta\nco-tc\nwh-qp
tb-vc\ntd-yn";
    let (network, computers) = parse_input(input);

    assert_eq!(4, connections_to(&"kh".to_string(), &network).len());
    assert_eq!(HashSet::from(["tc".to_string(),"qp".to_string(),"ub".to_string(), "ta".to_string()]),
    connections_to(&"kh".to_string(), &network).iter().map(|c| c.other(&"kh".to_string()).unwrap()).collect::<HashSet<String>>() );
    assert_eq!(7, trip_conns_w_t(&network, &computers).len())

    }
}